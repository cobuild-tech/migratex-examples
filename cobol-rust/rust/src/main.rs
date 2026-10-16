use std::io::{self, BufWriter};
use std::process::ExitCode;

use banco::banco::{Banco, Mode};
use banco::files::Files;

/// Runs in the current directory, like the COBOL program: the six `.dat`
/// files live next to it. `BANCO_COMPAT=1` reproduces the original's bugs.
fn main() -> ExitCode {
    let mode = match std::env::var("BANCO_COMPAT").as_deref() {
        Ok("1") => Mode::Compat,
        _ => Mode::Fixed,
    };
    let stdin = io::stdin().lock();
    let stdout = BufWriter::new(io::stdout().lock());
    match Banco::new(stdin, stdout, Files::new("."), mode).run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("banco: {e}");
            ExitCode::FAILURE
        }
    }
}
