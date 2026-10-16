//! Replays every session the COBOL programs were recorded on (see
//! `cobol/capture-golden.sh`) in compat mode and checks that the port prints
//! the same bytes and leaves the same data files behind, step by step.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use banco::pic::Pic;

const DATA_FILES: [&str; 6] = [
    "clientes.dat",
    "cuentas.dat",
    "tarjetas.dat",
    "movimientos.dat",
    "config.dat",
    "empleados.dat",
];
const CLOCK: &str = "2026/10/16 09:30:15.420000000";

fn golden() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden")
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("banco-golden-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    for f in DATA_FILES {
        fs::write(dir.join(f), b"").unwrap();
    }
    dir
}

/// The first line where two outputs part ways, for a readable failure.
fn first_difference(expected: &[u8], actual: &[u8]) -> String {
    let (e, a) = (
        String::from_utf8_lossy(expected),
        String::from_utf8_lossy(actual),
    );
    for (i, (le, la)) in e.lines().zip(a.lines()).enumerate() {
        if le != la {
            return format!("line {}:\n  cobol: {le:?}\n  rust:  {la:?}", i + 1);
        }
    }
    format!(
        "lengths differ: cobol {} bytes, rust {} bytes",
        expected.len(),
        actual.len()
    )
}

fn run_case(case: &Path) -> Vec<String> {
    let name = case.file_name().unwrap().to_string_lossy().to_string();
    let work = scratch(&name);
    let mut steps: Vec<_> = fs::read_dir(case)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| p.extension().is_some_and(|x| x == "in"))
        .collect();
    steps.sort();

    let mut failures = Vec::new();
    for step in steps {
        let step_name = step.file_stem().unwrap().to_string_lossy().to_string();
        let program = step_name.split_once('-').unwrap().1;
        let exe = match program {
            "banco" => env!("CARGO_BIN_EXE_banco"),
            "intereses" => env!("CARGO_BIN_EXE_intereses"),
            other => panic!("{name}/{step_name}: unknown program {other}"),
        };
        let mut child = Command::new(exe)
            .current_dir(&work)
            .env("COB_CURRENT_DATE", CLOCK)
            .env("BANCO_COMPAT", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        child
            .stdin
            .take()
            .unwrap()
            .write_all(&fs::read(&step).unwrap())
            .unwrap();
        let output = child.wait_with_output().unwrap();

        let expected = case.join("expected").join(&step_name);
        let want = fs::read(expected.join("stdout.txt")).unwrap();
        if want != output.stdout {
            failures.push(format!(
                "{name}/{step_name} stdout, {}",
                first_difference(&want, &output.stdout)
            ));
        }
        for f in DATA_FILES {
            if fs::read(expected.join(f)).unwrap() != fs::read(work.join(f)).unwrap() {
                failures.push(format!("{name}/{step_name}: {f} differs"));
            }
        }
    }
    let _ = fs::remove_dir_all(&work);
    failures
}

#[test]
fn every_session_matches_the_cobol_run_byte_for_byte() {
    let mut cases: Vec<_> = fs::read_dir(golden().join("cases"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    cases.sort();
    assert!(!cases.is_empty());
    let failures: Vec<String> = cases.iter().flat_map(|c| run_case(c)).collect();
    assert!(
        failures.is_empty(),
        "{} difference(s):\n{}",
        failures.len(),
        failures.join("\n")
    );
}

#[test]
fn number_parsing_matches_gnucobol_accept() {
    let dir = golden().join("accept");
    let inputs = fs::read(dir.join("input.txt")).unwrap();
    let expected = fs::read_to_string(dir.join("expected.txt")).unwrap();
    let fields = [
        Pic::new(12, 2),
        Pic::new(10, 0),
        Pic::new(8, 0),
        Pic::new(3, 3),
    ];

    let mut checked = 0;
    let mut failures = Vec::new();
    for (line, want) in inputs.split(|&b| b == b'\n').zip(expected.lines()) {
        let got: Vec<String> = fields
            .iter()
            .map(|p| String::from_utf8(p.show(p.accept(line))).unwrap())
            .collect();
        let got = got.join("|");
        if got != want {
            failures.push(format!(
                "{:?}: cobol {want}, rust {got}",
                String::from_utf8_lossy(line)
            ));
        }
        checked += 1;
    }
    assert_eq!(checked, expected.lines().count());
    assert!(
        failures.is_empty(),
        "{} of {checked} inputs differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}
