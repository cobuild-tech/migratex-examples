//! Replays a JSP file the way Jasper's generated servlet writes it.
//!
//! Jasper copies the text between `<% %>` tags to the response unchanged,
//! CRLF line endings included, so the compat pages are written from the
//! original `.jsp` files rather than from copies of their HTML. The Rust code
//! for each page walks the file's tags in order (`code`, `expr`, `skip`) and
//! does what the Java in each tag did. Walking past a tag without handling it
//! panics, so a page can't silently drop a piece of logic.

/// One piece of a JSP file.
#[derive(Clone, Copy, Debug)]
enum Part {
    Text(&'static str),
    /// `<% ... %>`
    Scriptlet(&'static str),
    /// `<%= ... %>`
    Expr(&'static str),
}

fn split(src: &'static str) -> Vec<Part> {
    let mut parts = Vec::new();
    let mut rest = src;
    while let Some(start) = rest.find("<%") {
        if start > 0 {
            parts.push(Part::Text(&rest[..start]));
        }
        let end = start + rest[start..].find("%>").expect("unclosed JSP tag");
        let tag = &rest[start + 2..end];
        if let Some(java) = tag.strip_prefix('=') {
            parts.push(Part::Expr(java));
        } else if !tag.starts_with('@') {
            // Directives (`<%@ page import ... %>`) write nothing.
            parts.push(Part::Scriptlet(tag));
        }
        rest = &rest[end + 2..];
    }
    if !rest.is_empty() {
        parts.push(Part::Text(rest));
    }
    parts
}

/// The response charset of a page, from its `contentType` directive.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Charset {
    /// No directive: Jasper's default.
    Latin1,
    Utf8,
}

impl Charset {
    pub fn content_type(self) -> &'static str {
        match self {
            Charset::Latin1 => "text/html;charset=ISO-8859-1",
            Charset::Utf8 => "text/html;charset=UTF-8",
        }
    }

    /// Java's encoder writes `?` for a character the charset doesn't have.
    pub fn encode(self, text: &str) -> Vec<u8> {
        match self {
            Charset::Utf8 => text.as_bytes().to_vec(),
            Charset::Latin1 => text
                .chars()
                .map(|c| u8::try_from(u32::from(c)).unwrap_or(b'?'))
                .collect(),
        }
    }
}

/// A page being written: the JSP's parts and a cursor into them.
pub struct Page {
    parts: Vec<Part>,
    next: usize,
    out: String,
    pub charset: Charset,
}

impl Page {
    pub fn new(src: &'static str, charset: Charset) -> Page {
        Page {
            parts: split(src),
            next: 0,
            out: String::new(),
            charset,
        }
    }

    /// `out.print(...)` from inside a scriptlet.
    pub fn print(&mut self, s: &str) {
        self.out.push_str(s);
    }

    /// `out.println(...)`: the JVM's line separator, `\n` on Linux.
    pub fn println(&mut self, s: &str) {
        self.out.push_str(s);
        self.out.push('\n');
    }

    /// Writes text up to the next tag.
    pub fn flush(&mut self) {
        while let Some(Part::Text(t)) = self.parts.get(self.next) {
            // The app is deployed as ROOT, so the context path is empty.
            self.out
                .push_str(&t.replace("${pageContext.request.contextPath}", ""));
            self.next += 1;
        }
    }

    fn take(&mut self, java: &str) -> Part {
        let part = self.parts[self.next];
        let (Part::Scriptlet(code) | Part::Expr(code)) = part else {
            unreachable!()
        };
        assert!(
            code.contains(java),
            "expected a tag containing {java:?}, found {code:?}"
        );
        self.next += 1;
        part
    }

    /// Writes the text before the next scriptlet, which must contain `java`.
    /// The caller then does what that Java did.
    pub fn code(&mut self, java: &str) {
        self.flush();
        assert!(matches!(self.take(java), Part::Scriptlet(_)));
    }

    /// Writes the text before the next `<%= %>`, which must contain `java`,
    /// then its value. Java prints `null` for a null String.
    pub fn expr(&mut self, java: &str, value: impl std::fmt::Display) {
        self.flush();
        assert!(matches!(self.take(java), Part::Expr(_)));
        use std::fmt::Write;
        write!(self.out, "{value}").unwrap();
    }

    /// Discards everything up to and including the scriptlet containing
    /// `java`: the body of an `if` that wasn't taken.
    pub fn skip(&mut self, java: &str) {
        loop {
            match self.parts[self.next] {
                Part::Scriptlet(code) if code.contains(java) => break,
                _ => self.next += 1,
            }
        }
        self.next += 1;
    }

    /// Where the cursor is, to go back to for each pass of a `while` loop.
    pub fn mark(&self) -> usize {
        self.next
    }

    pub fn rewind(&mut self, mark: usize) {
        self.next = mark;
    }

    /// Writes the rest of the file, which must have no tags left.
    pub fn finish(mut self) -> Vec<u8> {
        self.flush();
        assert!(
            self.next == self.parts.len(),
            "unhandled tag {:?}",
            self.parts[self.next]
        );
        self.charset.encode(&self.out)
    }
}

/// A JSP file's lines, for the excerpt in a JasperException.
pub fn lines(src: &str) -> Vec<&str> {
    src.split('\n')
        .map(|l| l.strip_suffix('\r').unwrap_or(l))
        .collect()
}
