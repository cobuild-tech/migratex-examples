//! `PIC 9(n)V9(m)` fields: unsigned zoned decimals.
//!
//! Every numeric field in the COBOL program is unsigned `DISPLAY`, so a value
//! is held here as a `u64` count of its smallest unit (cents for `V99`,
//! thousandths for `V999`). [`Pic`] carries the layout and copies the three
//! GnuCOBOL behaviours the program depends on: how a value is stored after
//! arithmetic, how it prints, and how `ACCEPT` turns typed text into a number.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pic {
    /// Digits before the implied decimal point.
    pub int: u32,
    /// Digits after it.
    pub scale: u32,
}

impl Pic {
    pub const fn new(int: u32, scale: u32) -> Pic {
        Pic { int, scale }
    }

    /// Bytes the field takes in a record (one per digit, no sign, no point).
    pub const fn size(self) -> usize {
        (self.int + self.scale) as usize
    }

    /// Stores a result the way `MOVE`, `ADD`, `SUBTRACT` and `COMPUTE` do with
    /// no `ROUNDED` and no `ON SIZE ERROR`: an unsigned field drops the sign,
    /// and digits that don't fit on the left are cut off. `ADD 1` to a
    /// `PIC 99` holding 99 gives 0.
    pub fn store(self, value: i128) -> u64 {
        (value.unsigned_abs() % 10u128.pow(self.int + self.scale)) as u64
    }

    /// What `DISPLAY` prints: every digit, zero-padded, with a `.` at the
    /// implied decimal point. 825.50 in `PIC 9(12)V99` is `000000000825.50`.
    pub fn show(self, value: u64) -> Vec<u8> {
        let digits = format!("{:0width$}", value, width = self.size());
        let (int, frac) = digits.split_at(self.int as usize);
        let mut out = int.as_bytes().to_vec();
        if self.scale > 0 {
            out.push(b'.');
            out.extend_from_slice(frac.as_bytes());
        }
        out
    }

    /// The record form: digits only.
    pub fn encode(self, value: u64, out: &mut Vec<u8>) {
        out.extend_from_slice(format!("{:0width$}", value, width = self.size()).as_bytes());
    }

    /// Reads the record form. A non-digit byte counts as its low four bits,
    /// which is what GnuCOBOL arithmetic sees (a space reads as 0).
    pub fn decode(self, bytes: &[u8]) -> u64 {
        bytes
            .iter()
            .fold(0u64, |acc, b| acc * 10 + u64::from(b & 0x0F) % 10)
    }

    /// `ACCEPT` into this field, copied from libcob 3.2: the line is first cut
    /// to the field's size, then moved the way `cob_move_alphanum_to_display`
    /// does it. Some results look odd, but the program relies on them:
    ///
    /// - commas are skipped, so `7,25` is 725.00 and `1,234.56` is 1234.56;
    /// - a sign is skipped, so `-5` is 5;
    /// - digits past the field's decimals are dropped, so `12.345` is 12.34;
    /// - anything else unexpected gives 0, unless the field is already full
    ///   when it's reached: `12a` is 12 in a `PIC 9(8)` but 0 in `PIC 9(12)V99`.
    pub fn accept(self, line: &[u8]) -> u64 {
        let src = &line[..line.len().min(self.size())];
        let mut out = vec![b'0'; self.size()];

        let mut i = 0;
        while i < src.len() && is_c_space(src[i]) {
            i += 1;
        }
        if i < src.len() && (src[i] == b'+' || src[i] == b'-') {
            i += 1;
        }

        // Line the integer digits up with the field: pad on the left, or skip
        // the high-order digits that don't fit.
        let int_digits = src[i..]
            .iter()
            .take_while(|&&c| c != b'.')
            .filter(|c| c.is_ascii_digit())
            .count();
        let int = self.int as usize;
        let mut o = 0;
        if int_digits < int {
            o = int - int_digits;
        } else {
            for _ in int..int_digits {
                while !src[i].is_ascii_digit() {
                    i += 1;
                }
                i += 1;
            }
        }

        let mut points = 0;
        while i < src.len() && o < out.len() {
            match src[i] {
                c @ b'0'..=b'9' => {
                    out[o] = c;
                    o += 1;
                }
                b'.' => {
                    points += 1;
                    if points > 1 {
                        return 0;
                    }
                }
                c if is_c_space(c) || c == b',' => {}
                _ => return 0,
            }
            i += 1;
        }
        self.decode(&out)
    }
}

/// C's `isspace` in the "C" locale.
fn is_c_space(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0B | 0x0C | b'\r')
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMPORTE: Pic = Pic::new(12, 2);
    const ID: Pic = Pic::new(8, 0);

    #[test]
    fn store_cuts_high_order_digits_and_drops_the_sign() {
        let p = Pic::new(2, 2);
        assert_eq!(p.store(10050), 50); // 99.50 + 1 -> 00.50
        assert_eq!(p.store(3 * 100 - 5 * 100), 200); // 3 - 5 -> 02.00
    }

    #[test]
    fn show_puts_the_point_where_the_v_is() {
        assert_eq!(IMPORTE.show(82550), b"000000000825.50");
        assert_eq!(Pic::new(3, 3).show(2500), b"002.500");
        assert_eq!(Pic::new(10, 0).show(100001), b"0000100001");
    }

    #[test]
    fn accept_matches_gnucobol_on_the_awkward_cases() {
        assert_eq!(IMPORTE.accept(b"100.50"), 10050);
        assert_eq!(IMPORTE.accept(b"7,25"), 72500);
        assert_eq!(IMPORTE.accept(b"-5"), 500);
        assert_eq!(IMPORTE.accept(b"12.345"), 1234);
        assert_eq!(IMPORTE.accept(b"abc"), 0);
        assert_eq!(IMPORTE.accept(b"1.2.3"), 0);
        assert_eq!(IMPORTE.accept(b"12a"), 0);
        assert_eq!(ID.accept(b"12a"), 12);
        assert_eq!(ID.accept(b"1e3"), 0);
        assert_eq!(IMPORTE.accept(b"1234567890123"), 23456789012300);
        assert_eq!(ID.accept(b"100000000"), 10000000);
    }

    #[test]
    fn decode_reads_what_encode_wrote() {
        let mut buf = Vec::new();
        IMPORTE.encode(82550, &mut buf);
        assert_eq!(buf, b"00000000082550");
        assert_eq!(IMPORTE.decode(&buf), 82550);
    }
}
