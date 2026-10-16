//! `ACCEPT ... FROM DATE YYYYMMDD` and `ACCEPT ... FROM TIME`.
//!
//! GnuCOBOL lets a test pin the clock with `COB_CURRENT_DATE`. The port reads
//! the same variable, so one setting drives both programs and their dates,
//! times, PINs and CVVs match.

use chrono::{Datelike, Local, Timelike};

pub struct Now {
    /// `YYYYMMDD`
    pub date: [u8; 8],
    /// `HHMMSSss` (hundredths of a second)
    pub time: [u8; 8],
}

pub fn now() -> Now {
    if let Some(fixed) = std::env::var("COB_CURRENT_DATE")
        .ok()
        .and_then(|s| parse_fixed(&s))
    {
        return fixed;
    }
    let t = Local::now();
    stamp(
        t.year() as u32,
        t.month(),
        t.day(),
        t.hour(),
        t.minute(),
        t.second(),
        t.nanosecond().min(999_999_999) / 10_000_000,
    )
}

fn stamp(y: u32, mo: u32, d: u32, h: u32, mi: u32, s: u32, cs: u32) -> Now {
    let mut date = [0u8; 8];
    let mut time = [0u8; 8];
    date.copy_from_slice(format!("{y:04}{mo:02}{d:02}").as_bytes());
    time.copy_from_slice(format!("{h:02}{mi:02}{s:02}{cs:02}").as_bytes());
    Now { date, time }
}

/// The form the tests use: `YYYY/MM/DD HH:MM:SS` with optional `.nnnnnnnnn`.
fn parse_fixed(s: &str) -> Option<Now> {
    let num = |r: std::ops::Range<usize>| s.get(r)?.parse::<u32>().ok();
    let nanos = match s.get(19..) {
        Some(rest) if rest.starts_with('.') => {
            let digits: String = rest[1..].chars().take(9).collect();
            digits.parse::<u32>().ok()?
        }
        _ => 0,
    };
    Some(stamp(
        num(0..4)?,
        num(5..7)?,
        num(8..10)?,
        num(11..13)?,
        num(14..16)?,
        num(17..19)?,
        nanos / 10_000_000,
    ))
}
