//! The parts of Java 8 that FlyAway's behaviour depends on: `null` Strings,
//! `Integer.parseInt`, `LocalDate.parse`, and exceptions with the stack
//! frames Tomcat prints for them.
//!
//! The line numbers in the frames are those of OpenJDK 1.8.0_282, the JVM in
//! the `tomcat:9.0.44-jdk8-openjdk` image the golden pages were recorded on.

/// A Java String that may be `null`. Concatenation and `<%= %>` print `null`.
pub type JString = Option<String>;

pub fn show(s: &JString) -> &str {
    s.as_deref().unwrap_or("null")
}

/// A thrown exception: what Tomcat's error page prints of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Throwable {
    pub class: &'static str,
    pub message: Option<String>,
    pub frames: Vec<String>,
    pub cause: Option<Box<Throwable>>,
}

impl Throwable {
    pub fn new(class: &'static str, message: Option<String>, frames: &[&str]) -> Throwable {
        Throwable {
            class,
            message,
            frames: frames.iter().map(|f| f.to_string()).collect(),
            cause: None,
        }
    }

    pub fn null_pointer(frames: &[&str]) -> Throwable {
        Throwable::new("java.lang.NullPointerException", None, frames)
    }

    /// Adds the caller's frames, to this exception and every cause, as
    /// the exception travels up the stack.
    pub fn at(mut self, frames: &[&str]) -> Throwable {
        self.frames.extend(frames.iter().map(|f| f.to_string()));
        if let Some(cause) = self.cause.take() {
            self.cause = Some(Box::new(cause.at(frames)));
        }
        self
    }

    /// `Throwable.toString()`.
    pub fn header(&self) -> String {
        match &self.message {
            Some(m) => format!("{}: {m}", self.class),
            None => self.class.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Integer.parseInt

/// The first code point of each run of ten digits that `Character.digit(c,
/// 10)` accepts, printed by Java 1.8.0_282 itself. `parseInt` reads UTF-16
/// units, so digits outside the BMP never count.
const DIGIT_ZEROS: [u16; 35] = [
    0x0030, 0x0660, 0x06F0, 0x07C0, 0x0966, 0x09E6, 0x0A66, 0x0AE6, 0x0B66, 0x0BE6, 0x0C66, 0x0CE6,
    0x0D66, 0x0E50, 0x0ED0, 0x0F20, 0x1040, 0x1090, 0x17E0, 0x1810, 0x1946, 0x19D0, 0x1A80, 0x1A90,
    0x1B50, 0x1BB0, 0x1C40, 0x1C50, 0xA620, 0xA8D0, 0xA900, 0xA9D0, 0xAA50, 0xABF0, 0xFF10,
];

/// `Character.digit(ch, 10)`.
fn digit(ch: u16) -> i32 {
    DIGIT_ZEROS
        .iter()
        .find(|&&zero| (zero..zero + 10).contains(&ch))
        .map_or(-1, |&zero| i32::from(ch - zero))
}

/// `Integer.parseInt(s)`, line for line, so each failure throws from the
/// same line as Java's.
pub fn parse_int(s: Option<&str>) -> Result<i32, Throwable> {
    let outer = "java.lang.Integer.parseInt(Integer.java:615)";
    let Some(s) = s else {
        return Err(Throwable::new(
            "java.lang.NumberFormatException",
            Some("null".into()),
            &["java.lang.Integer.parseInt(Integer.java:542)", outer],
        ));
    };
    let for_input = |line: u32| {
        Throwable::new(
            "java.lang.NumberFormatException",
            Some(format!("For input string: \"{s}\"")),
            &[
                "java.lang.NumberFormatException.forInputString(NumberFormatException.java:65)",
                &format!("java.lang.Integer.parseInt(Integer.java:{line})"),
                outer,
            ],
        )
    };
    let chars: Vec<u16> = s.encode_utf16().collect();
    let len = chars.len();
    if len == 0 {
        return Err(for_input(592));
    }
    let mut result: i32 = 0;
    let mut negative = false;
    let mut i = 0;
    let mut limit = -i32::MAX;
    let first = chars[0];
    if first < u16::from(b'0') {
        if first == u16::from(b'-') {
            negative = true;
            limit = i32::MIN;
        } else if first != u16::from(b'+') {
            return Err(for_input(569));
        }
        if len == 1 {
            return Err(for_input(572));
        }
        i += 1;
    }
    let multmin = limit / 10;
    while i < len {
        let d = digit(chars[i]);
        i += 1;
        if d < 0 {
            return Err(for_input(580));
        }
        if result < multmin {
            return Err(for_input(583));
        }
        result *= 10;
        if result < limit + d {
            return Err(for_input(587));
        }
        result -= d;
    }
    Ok(if negative { result } else { -result })
}

// ---------------------------------------------------------------------------
// LocalDate.parse

const PARSE_FRAMES: [&str; 3] = [
    "java.time.format.DateTimeFormatter.parse(DateTimeFormatter.java:1851)",
    "java.time.LocalDate.parse(LocalDate.java:400)",
    "java.time.LocalDate.parse(LocalDate.java:385)",
];

/// The frames between `IsoChronology.resolveYMD` and `LocalDate.parse`.
const RESOLVE_FRAMES: [&str; 12] = [
    "java.time.chrono.IsoChronology.resolveYMD(IsoChronology.java:123)",
    "java.time.chrono.AbstractChronology.resolveDate(AbstractChronology.java:472)",
    "java.time.chrono.IsoChronology.resolveDate(IsoChronology.java:492)",
    "java.time.chrono.IsoChronology.resolveDate(IsoChronology.java:123)",
    "java.time.format.Parsed.resolveDateFields(Parsed.java:351)",
    "java.time.format.Parsed.resolveFields(Parsed.java:257)",
    "java.time.format.Parsed.resolve(Parsed.java:244)",
    "java.time.format.DateTimeParseContext.toResolved(DateTimeParseContext.java:331)",
    "java.time.format.DateTimeFormatter.parseResolved0(DateTimeFormatter.java:1955)",
    "java.time.format.DateTimeFormatter.parse(DateTimeFormatter.java:1851)",
    "java.time.LocalDate.parse(LocalDate.java:400)",
    "java.time.LocalDate.parse(LocalDate.java:385)",
];

/// A date as `LocalDate` holds it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalDate {
    pub year: i64,
    pub month: u32,
    pub day: u32,
}

/// `NumberPrinterParser.parse` for one field of `ISO_LOCAL_DATE`, in strict
/// mode. Returns the value and the position after it, or the error position.
fn parse_number(
    text: &[u16],
    mut position: usize,
    min_width: usize,
    max_width: usize,
    year: bool,
) -> Result<(i64, usize), usize> {
    let length = text.len();
    if position == length {
        return Err(position);
    }
    let sign = text[position];
    let (mut negative, mut positive) = (false, false);
    if sign == u16::from(b'+') || sign == u16::from(b'-') {
        // The year is SignStyle.EXCEEDS_PAD, which takes either sign. Month
        // and day are NOT_NEGATIVE, which in strict mode takes neither.
        if !year {
            return Err(position);
        }
        negative = sign == u16::from(b'-');
        positive = !negative;
        position += 1;
    }
    let min_end = position + min_width;
    if min_end > length {
        return Err(position);
    }
    let max_end = (position + max_width).min(length);
    let mut total: i64 = 0;
    let mut pos = position;
    while pos < max_end {
        let ch = text[pos];
        pos += 1;
        // DecimalStyle.STANDARD: ASCII digits only.
        let digit = i64::from(ch) - i64::from(b'0');
        if !(0..=9).contains(&digit) {
            pos -= 1;
            if pos < min_end {
                return Err(position);
            }
            break;
        }
        total = total * 10 + digit;
    }
    if negative {
        if total == 0 {
            // "-0000" is not a year.
            return Err(position - 1);
        }
        total = -total;
    } else if year {
        let parsed = pos - position;
        if positive && parsed <= min_width {
            // A '+' is only allowed on a year of more than four digits...
            return Err(position - 1);
        }
        if !positive && parsed > min_width {
            // ...and required on one.
            return Err(position);
        }
    }
    Ok((total, pos))
}

fn parse_literal(text: &[u16], position: usize, literal: u8) -> Result<usize, usize> {
    match text.get(position) {
        Some(&c) if c == u16::from(literal) => Ok(position + 1),
        _ => Err(position),
    }
}

pub fn is_leap(year: i64) -> bool {
    (year & 3) == 0 && (year % 100 != 0 || year % 400 == 0)
}

const MONTHS: [&str; 12] = [
    "JANUARY",
    "FEBRUARY",
    "MARCH",
    "APRIL",
    "MAY",
    "JUNE",
    "JULY",
    "AUGUST",
    "SEPTEMBER",
    "OCTOBER",
    "NOVEMBER",
    "DECEMBER",
];

/// `LocalDate.parse(text)`, with `DateTimeFormatter.ISO_LOCAL_DATE`.
pub fn parse_local_date(s: &str) -> Result<LocalDate, Throwable> {
    let text: Vec<u16> = s.encode_utf16().collect();
    // DateTimeFormatter shortens the text in its messages.
    let abbr = if text.len() > 64 {
        format!("{}...", String::from_utf16_lossy(&text[..64]))
    } else {
        s.to_string()
    };
    let parse_error = |message: String, line: u32| {
        let mut frames = vec![format!(
            "java.time.format.DateTimeFormatter.parseResolved0(DateTimeFormatter.java:{line})"
        )];
        frames.extend(PARSE_FRAMES.iter().map(|f| f.to_string()));
        Throwable {
            class: "java.time.format.DateTimeParseException",
            message: Some(message),
            frames,
            cause: None,
        }
    };

    let fields = (|| {
        let (year, p) = parse_number(&text, 0, 4, 10, true)?;
        let p = parse_literal(&text, p, b'-')?;
        let (month, p) = parse_number(&text, p, 2, 2, false)?;
        let p = parse_literal(&text, p, b'-')?;
        let (day, p) = parse_number(&text, p, 2, 2, false)?;
        Ok::<_, usize>((year, month, day, p))
    })();
    let (year, month, day, end) = match fields {
        Err(index) => {
            return Err(parse_error(
                format!("Text '{abbr}' could not be parsed at index {index}"),
                1949,
            ))
        }
        Ok(fields) => fields,
    };
    if end < text.len() {
        return Err(parse_error(
            format!("Text '{abbr}' could not be parsed, unparsed text found at index {end}"),
            1952,
        ));
    }

    // Resolving, in ResolverStyle.STRICT: each field's range, then the date.
    let resolve_error = |message: String, first: &[&str]| {
        let mut frames: Vec<String> = first.iter().map(|f| f.to_string()).collect();
        frames.extend(RESOLVE_FRAMES.iter().map(|f| f.to_string()));
        let cause = Throwable {
            class: "java.time.DateTimeException",
            message: Some(message.clone()),
            frames,
            cause: None,
        };
        let mut frames = vec![
            "java.time.format.DateTimeFormatter.createError(DateTimeFormatter.java:1920)"
                .to_string(),
            "java.time.format.DateTimeFormatter.parse(DateTimeFormatter.java:1855)".to_string(),
        ];
        frames.extend(PARSE_FRAMES[1..].iter().map(|f| f.to_string()));
        Throwable {
            class: "java.time.format.DateTimeParseException",
            message: Some(format!("Text '{abbr}' could not be parsed: {message}")),
            frames,
            cause: Some(Box::new(cause)),
        }
    };
    let range = |field: &str, valid: &str, value: i64, line: u32| {
        resolve_error(
            format!("Invalid value for {field} (valid values {valid}): {value}"),
            &[
                "java.time.temporal.ValueRange.checkValidIntValue(ValueRange.java:330)",
                "java.time.temporal.ChronoField.checkValidIntValue(ChronoField.java:722)",
                &format!("java.time.chrono.IsoChronology.resolveYMD(IsoChronology.java:{line})"),
            ],
        )
    };
    if !(-999_999_999..=999_999_999).contains(&year) {
        return Err(range("Year", "-999999999 - 999999999", year, 544));
    }
    if !(1..=12).contains(&month) {
        return Err(range("MonthOfYear", "1 - 12", month, 550));
    }
    if !(1..=31).contains(&day) {
        return Err(range("DayOfMonth", "1 - 28/31", day, 551));
    }
    let days_in_month = match month {
        2 if is_leap(year) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if day > days_in_month {
        let of = |line: u32| {
            [
                format!("java.time.LocalDate.create(LocalDate.java:{line})"),
                "java.time.LocalDate.of(LocalDate.java:269)".to_string(),
                "java.time.chrono.IsoChronology.resolveYMD(IsoChronology.java:560)".to_string(),
            ]
        };
        let (message, frames) = if day == 29 {
            (
                format!("Invalid date 'February 29' as '{year}' is not a leap year"),
                of(429),
            )
        } else {
            (
                format!("Invalid date '{} {day}'", MONTHS[month as usize - 1]),
                of(431),
            )
        };
        let frames: Vec<&str> = frames.iter().map(String::as_str).collect();
        return Err(resolve_error(message, &frames));
    }
    Ok(LocalDate {
        year,
        month: month as u32,
        day: day as u32,
    })
}

impl LocalDate {
    /// `LocalDate.toEpochDay()`.
    pub fn epoch_day(self) -> i64 {
        let (y, m) = (self.year, i64::from(self.month));
        let mut total = 365 * y;
        if y >= 0 {
            total += (y + 3) / 4 - (y + 99) / 100 + (y + 399) / 400;
        } else {
            total -= y / -4 - y / -100 + y / -400;
        }
        total += (367 * m - 362) / 12;
        total += i64::from(self.day) - 1;
        if m > 2 {
            total -= 1;
            if !is_leap(y) {
                total -= 1;
            }
        }
        total - 719_528
    }

    /// `getDayOfWeek().toString()`.
    pub fn day_of_week(self) -> &'static str {
        const DAYS: [&str; 7] = [
            "MONDAY",
            "TUESDAY",
            "WEDNESDAY",
            "THURSDAY",
            "FRIDAY",
            "SATURDAY",
            "SUNDAY",
        ];
        DAYS[(self.epoch_day() + 3).rem_euclid(7) as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_of_week_matches_java() {
        let d = |s| parse_local_date(s).unwrap().day_of_week();
        assert_eq!(d("2026-10-19"), "MONDAY");
        assert_eq!(d("1970-01-01"), "THURSDAY");
        assert_eq!(d("2000-02-29"), "TUESDAY");
        assert_eq!(d("0000-01-01"), "SATURDAY");
        assert_eq!(d("-0001-12-31"), "FRIDAY");
    }

    #[test]
    fn parse_int_accepts_what_java_accepts() {
        assert_eq!(parse_int(Some("+3")), Ok(3));
        assert_eq!(parse_int(Some("007")), Ok(7));
        assert_eq!(parse_int(Some("\u{663}")), Ok(3));
        assert_eq!(parse_int(Some("-2147483648")), Ok(i32::MIN));
        assert!(parse_int(Some("2147483648")).is_err());
        assert!(parse_int(Some("\u{1D7D1}")).is_err());
    }
}
