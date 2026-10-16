//! `DISPLAY` and `ACCEPT` on a plain terminal (no SCREEN SECTION).

use std::io::{self, BufRead, Write};

use crate::pic::Pic;
use crate::records::{text, Text};

/// Why a menu stopped early.
#[derive(Debug)]
pub enum Stop {
    /// Input ran out. GnuCOBOL hands `ACCEPT` a single space at end of input,
    /// so the original repeats "Opcion invalida" forever; the port stops.
    Eof,
    Io(io::Error),
}

impl From<io::Error> for Stop {
    fn from(e: io::Error) -> Self {
        Stop::Io(e)
    }
}

pub type Step = Result<(), Stop>;

pub struct Term<R, W> {
    input: R,
    out: W,
}

impl<R: BufRead, W: Write> Term<R, W> {
    pub fn new(input: R, out: W) -> Self {
        Term { input, out }
    }

    /// `DISPLAY a b c`: the parts back to back, then a newline.
    pub fn display(&mut self, parts: &[&[u8]]) -> io::Result<()> {
        for p in parts {
            self.out.write_all(p)?;
        }
        self.out.write_all(b"\n")
    }

    /// `DISPLAY ... NO ADVANCING`.
    pub fn prompt(&mut self, text: &str) -> io::Result<()> {
        self.out.write_all(text.as_bytes())
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.out.flush()
    }

    /// One line of input without its newline.
    fn line(&mut self) -> Result<Vec<u8>, Stop> {
        self.out.flush()?;
        let mut buf = Vec::new();
        if self.input.read_until(b'\n', &mut buf)? == 0 {
            return Err(Stop::Eof);
        }
        if buf.last() == Some(&b'\n') {
            buf.pop();
        }
        Ok(buf)
    }

    /// `ACCEPT` into a `PIC X(n)`.
    pub fn accept_x<const N: usize>(&mut self) -> Result<Text<N>, Stop> {
        Ok(text(&self.line()?))
    }

    /// `ACCEPT` into a `PIC 9(n)V9(m)`.
    pub fn accept_n(&mut self, pic: Pic) -> Result<u64, Stop> {
        Ok(pic.accept(&self.line()?))
    }
}

/// `display!(term, "Saldo: ", SALDO.show(x))`: anything that is bytes.
#[macro_export]
macro_rules! display {
    ($t:expr $(, $part:expr)* $(,)?) => {
        $t.display(&[$(AsRef::<[u8]>::as_ref(&$part)),*])
    };
}
