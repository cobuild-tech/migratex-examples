//! Replays every session in php/cases against the port in compat mode and
//! compares with what PHP sent: status, headers, body bytes, and the table.
//!
//! Needs MariaDB: `docker compose up -d db` in php-rust/.

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
            name.as_str() == "content-type" || name.as_str().starts_with("access-control-")
        })
        .map(|(name, value)| format!("{}: {}", name.as_str(), value.to_str().unwrap()))
        .collect();
    lines.sort();
    lines.iter().map(|l| format!("{l}\n")).collect()
}

#[tokio::test]
async fn every_session_matches_php() {
    let _lock = DB_LOCK.lock().await;
    let app = app(true);
    let mut cases: Vec<_> = fs::read_dir(php_dir().join("cases"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    cases.sort();
    let mut failures = Vec::new();

    for case in &cases {
        let name = case.file_stem().unwrap().to_str().unwrap();
        let expected = golden_dir().join(name);
        reset_db().await;
        let script = fs::read_to_string(case).unwrap();
        for (i, line) in script.lines().enumerate() {
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
                failures.push(format!(
                    "{where_}: body\n  got:  {}\n  want: {}",
                    String::from_utf8_lossy(&got.body),
                    String::from_utf8_lossy(&want_body)
                ));
            }
        }
        let want_table = fs::read_to_string(expected.join("students.tsv")).unwrap();
        let got_table = students_tsv().await;
        if got_table != want_table {
            failures.push(format!(
                "{name}: students table\n{got_table}!=\n{want_table}"
            ));
        }
    }

    assert!(cases.len() >= 20, "no cases found");
    assert!(
        failures.is_empty(),
        "{} differences:\n\n{}",
        failures.len(),
        failures.join("\n\n")
    );
}

/// Recorded by stopping the database: the `$e` typo in config/Database.php.
#[tokio::test]
async fn database_down_matches_php() {
    let app = students::router(students::App {
        pool: students::db::pool("mysql://root@127.0.0.1:1/studentdb").unwrap(),
        compat: true,
    });
    for script in students::SCRIPTS {
        let method = students::fixed::method_for(script);
        let got = send(&app, method.as_str(), &format!("/api/{script}"), Some("{}")).await;
        let want = fs::read(
            golden_dir()
                .join("database-down")
                .join(format!("{script}.body")),
        )
        .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&got.body),
            String::from_utf8_lossy(&want),
            "{script}"
        );
    }
}
