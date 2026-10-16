//! The PHP behaviour the API relies on without writing it down, written down.
//!
//! Only compat mode uses this module. Each function names the PHP it stands in
//! for, and the golden tests check it against PHP 7.3 with XAMPP's settings
//! (`display_errors = On`, `html_errors = On`).

use serde_json::Value;

/// Where the scripts lived on the recorded server. PHP prints these paths in
/// its notices and stack traces, so they are part of the response.
pub const ROOT: &str = "/var/www/html";
pub const MODEL: &str = "/var/www/html/models/Student.php";

/// `json_decode(file_get_contents("php://input"))`, without `$assoc`.
///
/// PHP gives `null` for an empty or invalid body. Duplicate keys keep the last
/// value, as serde_json does.
pub fn json_decode(body: &[u8]) -> Option<Value> {
    serde_json::from_slice(body).ok()
}

/// `isset($data->name)`: `$data` is an object, has the key, and it isn't null.
pub fn isset(data: &Option<Value>, name: &str) -> bool {
    matches!(data, Some(Value::Object(map)) if map.get(name).is_some_and(|v| !v.is_null()))
}

/// The script stopped. The output so far, error text included, is the body.
pub struct Halt;

/// What a script echoes, notices and fatal errors included.
pub struct Output {
    pub body: Vec<u8>,
    script: &'static str,
}

impl Output {
    pub fn new(script: &'static str) -> Self {
        Output {
            body: Vec::new(),
            script,
        }
    }

    pub fn script_path(&self) -> String {
        format!("{ROOT}/api/{}", self.script)
    }

    pub fn echo(&mut self, bytes: &[u8]) {
        self.body.extend_from_slice(bytes);
    }

    /// `echo json_encode(array('message' => $text))`
    pub fn message(&mut self, text: &str) {
        let json = json_encode_object(&[("message", text.as_bytes())]).unwrap_or_default();
        self.echo(json.as_bytes());
    }

    fn error(&mut self, kind: &str, message: &str, file: &str, line: u32) {
        let text = format!(
            "<br />\n<b>{kind}</b>:  {message} in <b>{file}</b> on line <b>{line}</b><br />\n"
        );
        self.echo(text.as_bytes());
    }

    pub fn notice(&mut self, message: &str, file: &str, line: u32) {
        self.error("Notice", message, file, line);
    }

    /// `$data->name` in an endpoint script, on `line`. Reading a missing
    /// property gives `null` and a notice.
    pub fn prop<'a>(
        &mut self,
        data: &'a Option<Value>,
        name: &str,
        line: u32,
    ) -> Option<&'a Value> {
        let file = self.script_path();
        match data {
            Some(Value::Object(map)) => {
                let value = map.get(name);
                if value.is_none() {
                    self.notice(
                        &format!("Undefined property: stdClass::${name}"),
                        &file,
                        line,
                    );
                }
                value.filter(|v| !v.is_null())
            }
            _ => {
                self.notice(
                    &format!("Trying to get property '{name}' of non-object"),
                    &file,
                    line,
                );
                None
            }
        }
    }

    /// `$stmt->bindParam(..., $value)` in the model, on `line`. With PDO's
    /// emulated prepares every value is sent as a quoted string, so this is
    /// PHP's string conversion. `None` is SQL `NULL`.
    pub fn bind(&mut self, value: Option<&Value>, line: u32) -> Result<Option<String>, Halt> {
        Ok(match value {
            None | Some(Value::Null) => None,
            Some(Value::Bool(true)) => Some("1".into()),
            Some(Value::Bool(false)) => Some(String::new()),
            Some(Value::Number(n)) => Some(number_to_string(n)),
            Some(Value::String(s)) => Some(s.clone()),
            Some(Value::Array(_)) => {
                self.notice("Array to string conversion", MODEL, line);
                Some("Array".into())
            }
            Some(Value::Object(_)) => {
                self.error(
                    "Recoverable fatal error",
                    "Object of class stdClass could not be converted to string",
                    MODEL,
                    line,
                );
                return Err(Halt);
            }
        })
    }

    /// A `PDOException` nobody catches, thrown by `$stmt->execute()` on
    /// `model_line` of `Student::$method()`, called from the script on
    /// `script_line`.
    pub fn uncaught_pdo_exception(
        &mut self,
        pdo_message: &str,
        model_line: u32,
        method: &str,
        script_line: u32,
    ) -> Halt {
        let script = self.script_path();
        let message = html_escape(&format!(
            "Uncaught PDOException: {pdo_message} in {MODEL}:{model_line}\n\
             Stack trace:\n\
             #0 {MODEL}({model_line}): PDOStatement->execute()\n\
             #1 {script}({script_line}): Student->{method}()\n\
             #2 {{main}}\n  thrown"
        ));
        self.error("Fatal error", &message, MODEL, model_line);
        Halt
    }

    /// `config/Database.php` when the connection fails. Its `catch` block reads
    /// `$e` but the exception is `$exp`, so instead of an error message PHP
    /// prints a notice and then dies calling a method on `null`.
    pub fn database_connect_failed(&mut self, script_line: u32) -> Halt {
        let database = format!("{ROOT}/config/Database.php");
        let script = self.script_path();
        self.notice("Undefined variable: e", &database, 16);
        let message = html_escape(&format!(
            "Uncaught Error: Call to a member function getMessage() on null in {database}:16\n\
             Stack trace:\n\
             #0 {script}({script_line}): Database->connect()\n\
             #1 {{main}}\n  thrown"
        ));
        self.error("Fatal error", &message, &database, 16);
        Halt
    }
}

/// PHP's `htmlspecialchars` with `ENT_COMPAT`, which `html_errors` applies.
fn html_escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            _ => out.push(c),
        }
    }
    out
}

/// A JSON number as PHP holds it after `json_decode`, converted to a string.
/// Integers that fit in 64 bits stay integers; anything else is a float.
fn number_to_string(n: &serde_json::Number) -> String {
    match n.as_i64() {
        Some(i) => i.to_string(),
        None => float_to_string(n.as_f64().unwrap_or(0.0)),
    }
}

/// `(string) $float` with `precision = 14`: `%.14G`, plus PHP's habit of
/// writing `1.0E+20` rather than `1E+20`.
pub fn float_to_string(f: f64) -> String {
    if f == 0.0 {
        return if f.is_sign_negative() {
            "-0".into()
        } else {
            "0".into()
        };
    }
    // 14 significant digits, as d.ddddddddddddde±x
    let sci = format!("{:.13e}", f.abs());
    let (mantissa, exp) = sci.split_once('e').unwrap();
    let exp: i32 = exp.parse().unwrap();
    let digits: String = mantissa.replace('.', "");
    let digits = digits.trim_end_matches('0');
    let sign = if f < 0.0 { "-" } else { "" };

    if !(-4..14).contains(&exp) {
        let (first, rest) = digits.split_at(1);
        let rest = if rest.is_empty() { "0" } else { rest };
        let exp_sign = if exp < 0 { "-" } else { "+" };
        return format!("{sign}{first}.{rest}E{exp_sign}{}", exp.abs());
    }
    if exp < 0 {
        let zeros = "0".repeat((-exp - 1) as usize);
        return format!("{sign}0.{zeros}{digits}");
    }
    let int_len = exp as usize + 1;
    if digits.len() <= int_len {
        format!("{sign}{digits}{}", "0".repeat(int_len - digits.len()))
    } else {
        format!("{sign}{}.{}", &digits[..int_len], &digits[int_len..])
    }
}

/// `json_encode` of a string with no flags: `/` becomes `\/` and everything
/// outside ASCII becomes `\uXXXX`. Invalid UTF-8 makes `json_encode` fail.
fn json_encode_str(bytes: &[u8], out: &mut String) -> Option<()> {
    let text = std::str::from_utf8(bytes).ok()?;
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '/' => out.push_str("\\/"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 || (c as u32) > 0x7f => {
                let mut units = [0u16; 2];
                for unit in c.encode_utf16(&mut units) {
                    out.push_str(&format!("\\u{unit:04x}"));
                }
            }
            c => out.push(c),
        }
    }
    out.push('"');
    Some(())
}

/// `json_encode(array('key' => 'value', ...))` where every value is a string,
/// which is all this API ever sends. `None` is `json_encode` returning `false`.
pub fn json_encode_object(fields: &[(&str, &[u8])]) -> Option<String> {
    let mut out = String::from("{");
    for (i, (key, value)) in fields.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        json_encode_str(key.as_bytes(), &mut out)?;
        out.push(':');
        json_encode_str(value, &mut out)?;
    }
    out.push('}');
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn floats_print_like_php_7() {
        let cases = [
            (1e20, "1.0E+20"),
            (1.5e-7, "1.5E-7"),
            (1.1, "1.1"),
            (0.1, "0.1"),
            (-0.0, "-0"),
            (15.9, "15.9"),
            (19.5, "19.5"),
            (9223372036854775808.0, "9.2233720368548E+18"),
            (0.0001, "0.0001"),
            (0.00001, "1.0E-5"),
            (12345678901234.0, "12345678901234"),
            (123456789012345.0, "1.2345678901234E+14"),
            (1.0 / 3.0, "0.33333333333333"),
            (-2.5, "-2.5"),
        ];
        for (f, want) in cases {
            assert_eq!(float_to_string(f), want, "{f:e}");
        }
    }

    #[test]
    fn strings_encode_like_json_encode() {
        let got = json_encode_object(&[("a", "Zoë \"Z\" 12/B\t😀".as_bytes())]).unwrap();
        assert_eq!(
            got,
            "{\"a\":\"Zo\\u00eb \\\"Z\\\" 12\\/B\\t\\ud83d\\ude00\"}"
        );
        assert_eq!(json_encode_object(&[("a", b"\xff")]), None);
    }
}
