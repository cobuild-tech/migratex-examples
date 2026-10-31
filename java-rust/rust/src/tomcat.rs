//! What Tomcat 9.0.44 does around the application: request parameters,
//! static files, and its error pages.

use crate::java::Throwable;
use axum::http::StatusCode;

/// A response, before it becomes an axum one.
pub struct Reply {
    pub status: StatusCode,
    pub headers: Vec<(&'static str, String)>,
    pub body: Vec<u8>,
}

impl Reply {
    pub fn ok(content_type: Option<&str>, body: Vec<u8>) -> Reply {
        Reply {
            status: StatusCode::OK,
            headers: content_type
                .map(|ct| ("content-type", ct.to_string()))
                .into_iter()
                .collect(),
            body,
        }
    }

    /// `response.sendRedirect(path)`: Tomcat 9 sends the path as given and
    /// drops anything already written.
    pub fn redirect(path: &str) -> Reply {
        Reply {
            status: StatusCode::FOUND,
            headers: vec![("location", path.to_string())],
            body: Vec::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Parameters

fn hex(b: u8) -> Option<u8> {
    (b as char).to_digit(16).map(|d| d as u8)
}

/// One `name=value` half: `+` is a space and `%XX` a byte. `None` for a bad
/// escape, which makes Tomcat drop the parameter.
fn url_decode(s: &str) -> Option<Vec<u8>> {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => out.push(b' '),
            b'%' => {
                let h = hex(*bytes.get(i + 1)?)?;
                let l = hex(*bytes.get(i + 2)?)?;
                out.push(h << 4 | l);
                i += 2;
            }
            b => out.push(b),
        }
        i += 1;
    }
    Some(out)
}

/// How the bytes of a parameter become a Java String.
#[derive(Clone, Copy)]
pub enum Decode {
    /// The query string: Tomcat 9's default URIEncoding.
    Utf8,
    /// A form body with no charset in its Content-Type: ISO-8859-1, so each
    /// byte becomes one char and UTF-8 text arrives as mojibake.
    Latin1,
}

fn add_params(params: &mut Vec<(String, String)>, raw: &str, decode: Decode) {
    for pair in raw.split('&').filter(|p| !p.is_empty()) {
        let (name, value) = pair.split_once('=').unwrap_or((pair, ""));
        let (Some(name), Some(value)) = (url_decode(name), url_decode(value)) else {
            continue;
        };
        let text = |b: Vec<u8>| match decode {
            Decode::Utf8 => String::from_utf8_lossy(&b).into_owned(),
            Decode::Latin1 => b.into_iter().map(char::from).collect(),
        };
        if !name.is_empty() {
            params.push((text(name), text(value)));
        }
    }
}

/// `request.getParameter(...)` for one request: the query string's
/// parameters first, then a form body's.
pub struct Params(Vec<(String, String)>);

impl Params {
    pub fn parse(query: Option<&str>, form_body: Option<&[u8]>) -> Params {
        let mut params = Vec::new();
        if let Some(q) = query {
            add_params(&mut params, q, Decode::Utf8);
        }
        if let Some(body) = form_body {
            // Latin-1 can decode any bytes, so this never fails.
            let raw: String = body.iter().map(|&b| char::from(b)).collect();
            add_params(&mut params, &raw, Decode::Latin1);
        }
        Params(params)
    }

    /// The first value, or `null`.
    pub fn get(&self, name: &str) -> Option<String> {
        self.0
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, v)| v.clone())
    }
}

// ---------------------------------------------------------------------------
// Static files, served by Tomcat's DefaultServlet

const FILES: [(&str, &[u8], &str); 6] = [
    (
        "/img/favicon.png",
        include_bytes!("../../java/web/img/favicon.png"),
        "image/png",
    ),
    (
        "/img/flyaway.png",
        include_bytes!("../../java/web/img/flyaway.png"),
        "image/png",
    ),
    (
        "/img/plane.png",
        include_bytes!("../../java/web/img/plane.png"),
        "image/png",
    ),
    (
        "/img/plane-flipped.png",
        include_bytes!("../../java/web/img/plane-flipped.png"),
        "image/png",
    ),
    (
        "/img/route.png",
        include_bytes!("../../java/web/img/route.png"),
        "image/png",
    ),
    (
        "/style/global.css",
        include_bytes!("../../java/web/style/global.css"),
        "text/css",
    ),
];

pub fn static_file(path: &str) -> Option<Reply> {
    FILES
        .iter()
        .find(|(p, _, _)| *p == path)
        .map(|(_, body, ct)| Reply::ok(Some(ct), body.to_vec()))
}

// ---------------------------------------------------------------------------
// Error pages, from Tomcat's ErrorReportValve

/// `Escape.htmlElementContent`.
pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            '/' => out.push_str("&#47;"),
            c => out.push(c),
        }
    }
    out
}

const STYLE: &str = "body {font-family:Tahoma,Arial,sans-serif;} h1, h2, h3, b {color:white;background-color:#525D76;} h1 {font-size:22px;} h2 {font-size:16px;} h3 {font-size:14px;} p {font-size:12px;} a {color:black;} .line {height:1px;background-color:#525D76;border:none;}";

fn describe(status: StatusCode) -> &'static str {
    match status.as_u16() {
        404 => "The origin server did not find a current representation for the target resource or is not willing to disclose that one exists.",
        405 => "The method received in the request-line is known by the origin server but not supported by the target resource.",
        500 => "The server encountered an unexpected condition that prevented it from fulfilling the request.",
        501 => "The server does not support the functionality required to fulfill the request.",
        _ => unreachable!("no page recorded for {status}"),
    }
}

/// How the exception printed: its toString and one tab-indented frame a line.
fn trace(t: &Throwable) -> String {
    let mut out = escape(&t.header());
    out.push('\n');
    for frame in &t.frames {
        out.push('\t');
        out.push_str(&escape(frame));
        out.push('\n');
    }
    out
}

fn page(status: StatusCode, message: Option<&str>, exception: Option<&Throwable>) -> String {
    let title = format!(
        "HTTP Status {} – {}",
        status.as_u16(),
        status.canonical_reason().unwrap()
    );
    let mut html = format!(
        "<!doctype html><html lang=\"en\"><head><title>{title}</title><style type=\"text/css\">{STYLE}</style></head><body><h1>{title}</h1><hr class=\"line\" /><p><b>Type</b> {}</p>",
        if exception.is_some() { "Exception Report" } else { "Status Report" }
    );
    if let Some(m) = message.filter(|m| !m.is_empty()) {
        html.push_str(&format!("<p><b>Message</b> {}</p>", escape(m)));
    }
    html.push_str(&format!("<p><b>Description</b> {}</p>", describe(status)));
    if let Some(e) = exception {
        html.push_str(&format!("<p><b>Exception</b></p><pre>{}</pre>", trace(e)));
        let mut cause = e.cause.as_deref();
        while let Some(c) = cause {
            html.push_str(&format!("<p><b>Root Cause</b></p><pre>{}</pre>", trace(c)));
            cause = c.cause.as_deref();
        }
        html.push_str("<p><b>Note</b> The full stack trace of the root cause is available in the server logs.</p>");
    }
    html.push_str("<hr class=\"line\" /><h3>Apache Tomcat/9.0.44</h3></body></html>");
    html
}

/// `latin1` is for a servlet that had already called `getWriter()`, which
/// fixes the response's charset to ISO-8859-1 before Tomcat writes its page,
/// so the dash in the title comes out as `?`.
fn reply(status: StatusCode, html: String, latin1: bool) -> Reply {
    let (charset, body) = if latin1 {
        ("ISO-8859-1", crate::jsp::Charset::Latin1.encode(&html))
    } else {
        ("utf-8", html.into_bytes())
    };
    Reply {
        status,
        headers: vec![
            ("content-type", format!("text/html;charset={charset}")),
            ("content-language", "en".to_string()),
        ],
        body,
    }
}

pub fn status_page(status: StatusCode, message: Option<&str>) -> Reply {
    reply(status, page(status, message, None), false)
}

pub fn not_found(path: &str) -> Reply {
    let message = if path.starts_with("/WEB-INF") || path.starts_with("/META-INF") {
        None
    } else if path.ends_with(".jsp") {
        Some(format!("JSP file [{path}] not found"))
    } else {
        Some(format!("The requested resource [{path}] is not available"))
    };
    status_page(StatusCode::NOT_FOUND, message.as_deref())
}

pub fn method_not_allowed(message: &str, allow: Option<&str>) -> Reply {
    let mut reply = status_page(StatusCode::METHOD_NOT_ALLOWED, Some(message));
    if let Some(allow) = allow {
        reply.headers.push(("allow", allow.to_string()));
    }
    reply
}

/// An exception thrown out of a servlet. The page's Message is the first
/// line of the exception's message.
pub fn exception(t: &Throwable, latin1: bool) -> Reply {
    let message = t.message.as_deref().map(|m| m.lines().next().unwrap_or(""));
    reply(
        StatusCode::INTERNAL_SERVER_ERROR,
        page(StatusCode::INTERNAL_SERVER_ERROR, message, Some(t)),
        latin1,
    )
}
