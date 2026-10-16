use std::io::{self, BufWriter};
use std::process::ExitCode;

use banco::files::Files;

/// Monthly interest posting, run in the folder that holds the data files.
fn main() -> ExitCode {
    let files = Files::new(".");
    let result = files
        .create_missing()
        .and_then(|_| banco::intereses::run(&files, BufWriter::new(io::stdout().lock())));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("intereses: {e}");
            ExitCode::FAILURE
        }
    }
}
