//! Rust port of [Banco-en-COBOL](https://github.com/Mel-Roses/Banco-en-COBOL).
//!
//! The port reads and writes the original's data files unchanged, prints the
//! same Spanish screens, and in [`banco::Mode::Compat`] matches the GnuCOBOL
//! build byte for byte. See the README for the mapping and the list of bugs
//! the default mode fixes.

pub mod banco;
pub mod clock;
pub mod files;
pub mod intereses;
pub mod pic;
pub mod records;
pub mod term;
