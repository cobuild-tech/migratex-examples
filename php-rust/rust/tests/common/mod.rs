//! Shared by the test files: a fresh database and a way to send a request.

#![allow(dead_code)]

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use sqlx::{Connection, Executor, MySqlConnection};
use std::path::PathBuf;
use students::{db, router, App};
use tower::ServiceExt;

pub fn database_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "mysql://root@127.0.0.1:3317/studentdb".into())
}

pub fn php_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../php")
}

/// Tests in one binary share the database, so they take turns.
pub static DB_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

/// Drops the database and loads db/studentdb.sql, as capture-golden.sh does.
pub async fn reset_db() {
    let url = database_url();
    let (server, name) = url.rsplit_once('/').unwrap();
    let mut conn = MySqlConnection::connect(server)
        .await
        .expect("cannot reach MariaDB");
    conn.execute(format!("DROP DATABASE IF EXISTS {name}; CREATE DATABASE {name}").as_str())
        .await
        .unwrap();
    conn.close().await.unwrap();

    let dump = std::fs::read_to_string(php_dir().join("db/studentdb.sql")).unwrap();
    let mut conn = MySqlConnection::connect(&url).await.unwrap();
    sqlx::raw_sql(&dump).execute(&mut conn).await.unwrap();
    conn.close().await.unwrap();
}

pub fn app(compat: bool) -> axum::Router {
    router(App {
        pool: db::pool(&database_url()).unwrap(),
        compat,
    })
}

pub struct Response {
    pub status: StatusCode,
    pub headers: axum::http::HeaderMap,
    pub body: Vec<u8>,
}

pub async fn send(app: &axum::Router, method: &str, path: &str, body: Option<&str>) -> Response {
    let request = Request::builder()
        .method(Method::from_bytes(method.as_bytes()).unwrap())
        .uri(path)
        // What curl's --data-binary sends.
        .header("content-type", "application/x-www-form-urlencoded")
        .body(Body::from(body.unwrap_or("").to_owned()))
        .unwrap();
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    Response {
        status,
        headers,
        body,
    }
}

/// The students table the way capture-golden.sh saves it.
pub async fn students_tsv() -> String {
    use sqlx::Row;
    let mut conn = MySqlConnection::connect(&database_url()).await.unwrap();
    let rows = sqlx::query("SELECT id, HEX(name), HEX(address), age FROM students ORDER BY id")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    // `mysql -B` prints the header only when there are rows.
    if rows.is_empty() {
        return String::new();
    }
    let mut out = String::from("id\tHEX(name)\tHEX(address)\tage\n");
    for row in rows {
        let id: u32 = row.get(0);
        let name: String = row.get(1);
        let address: String = row.get(2);
        let age: i32 = row.get(3);
        out.push_str(&format!("{id}\t{name}\t{address}\t{age}\n"));
    }
    out
}
