//! Replays every session in java/cases against the port in compat mode and
//! compares with what Tomcat sent: status, headers and body bytes.
//!
//! Needs MySQL with the flight table: `docker compose up -d db` in java-rust/.

mod common;

use common::*;
use std::fs;
use std::path::Path;

fn golden_dir() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden")
}

/// The headers capture-golden.sh keeps, in the same form.
fn header_lines(headers: &axum::http::HeaderMap) -> String {
    let mut lines: Vec<String> = headers
        .iter()
        .filter(|(name, _)| {
            matches!(
                name.as_str(),
                "content-type" | "content-language" | "location" | "allow"
            )
        })
        .map(|(name, value)| format!("{}: {}", name.as_str(), value.to_str().unwrap()))
        .collect();
    lines.sort();
    lines.iter().map(|l| format!("{l}\n")).collect()
}

#[tokio::test]
async fn every_session_matches_java() {
    let mut cases: Vec<_> = fs::read_dir(java_dir().join("cases"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    cases.sort();
    let mut failures = Vec::new();
    let mut requests = 0;

    for case in &cases {
        let name = case.file_stem().unwrap().to_str().unwrap();
        let expected = golden_dir().join(name);
        // Each session starts on a fresh Tomcat, so a fresh set of statics.
        let app = app(flyaway::Mode::compat());
        let script = fs::read_to_string(case).unwrap();
        for (i, line) in script.lines().enumerate() {
            requests += 1;
            let step = format!("{:02}", i + 1);
            let mut parts = line.splitn(3, ' ');
            let (method, path, body) = (parts.next().unwrap(), parts.next().unwrap(), parts.next());
            let got = send(&app, method, path, body).await;

            let want_status = fs::read_to_string(expected.join(format!("{step}.status"))).unwrap();
            let want_headers =
                fs::read_to_string(expected.join(format!("{step}.headers"))).unwrap();
            let want_body = fs::read(expected.join(format!("{step}.body"))).unwrap();
            let where_ = format!("{name} step {step} ({line})");
            if got.status.as_u16().to_string() != want_status.trim() {
                failures.push(format!(
                    "{where_}: status {} != {}",
                    got.status,
                    want_status.trim()
                ));
            }
            if header_lines(&got.headers) != want_headers {
                failures.push(format!(
                    "{where_}: headers\n{}!=\n{want_headers}",
                    header_lines(&got.headers)
                ));
            }
            if got.body != want_body {
                let at = got
                    .body
                    .iter()
                    .zip(&want_body)
                    .position(|(a, b)| a != b)
                    .unwrap_or(got.body.len().min(want_body.len()));
                let around = |b: &[u8]| {
                    String::from_utf8_lossy(&b[at.saturating_sub(80)..(at + 80).min(b.len())])
                        .into_owned()
                };
                failures.push(format!(
                    "{where_}: body differs at byte {at} ({} vs {} bytes)\n  got:  {:?}\n  want: {:?}",
                    got.body.len(),
                    want_body.len(),
                    around(&got.body),
                    around(&want_body)
                ));
            }
        }
    }

    assert!(requests >= 150, "only {requests} requests found");
    assert!(
        failures.is_empty(),
        "{} differences:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}
