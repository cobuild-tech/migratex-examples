//! One test per fix in the default mode. Each says what PHP did instead; the
//! golden tests show PHP really did it.
//!
//! Needs MariaDB: `docker compose up -d db` in php-rust/.

mod common;

use axum::http::StatusCode;
use common::*;
use serde_json::{json, Value};

async fn fresh() -> (tokio::sync::MutexGuard<'static, ()>, axum::Router) {
    let lock = DB_LOCK.lock().await;
    reset_db().await;
    (lock, app(false))
}

fn json(r: &Response) -> Value {
    serde_json::from_slice(&r.body)
        .unwrap_or_else(|_| panic!("not JSON: {}", String::from_utf8_lossy(&r.body)))
}

fn header<'a>(r: &'a Response, name: &str) -> &'a str {
    r.headers
        .get(name)
        .map(|v| v.to_str().unwrap())
        .unwrap_or("")
}

// PHP: `"id":"15"` and `"age":"23"`, because PDO returns strings.
#[tokio::test]
async fn numbers_are_numbers() {
    let (_lock, app) = fresh().await;
    let r = send(&app, "GET", "/api/get_one.php", Some(r#"{"id":16}"#)).await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(
        json(&r),
        json!({"id": 16, "name": "Joel", "address": "Delhi", "age": 23})
    );
}

// PHP: `{"message":"No records found!"}`, so clients had to handle two shapes.
#[tokio::test]
async fn empty_list_is_an_empty_array() {
    let (_lock, app) = fresh().await;
    for id in [15, 16, 17] {
        send(
            &app,
            "DELETE",
            "/api/delete.php",
            Some(&format!(r#"{{"id":{id}}}"#)),
        )
        .await;
    }
    let r = send(&app, "GET", "/api/get.php", None).await;
    assert_eq!(r.status, StatusCode::OK);
    assert_eq!(json(&r), json!([]));
}

// PHP: 200 with `{"message":"Error: incorrect Method!"}`.
#[tokio::test]
async fn wrong_method_is_405_with_allow() {
    let (_lock, app) = fresh().await;
    let r = send(&app, "GET", "/api/delete.php", Some(r#"{"id":15}"#)).await;
    assert_eq!(r.status, StatusCode::METHOD_NOT_ALLOWED);
    assert_eq!(header(&r, "allow"), "DELETE");
    assert_eq!(json(&r), json!({"message": "Error: incorrect Method!"}));
}

// PHP: `Access-Control-Allow-Methods: POST` on put.php and delete.php too.
#[tokio::test]
async fn allow_methods_names_the_real_method() {
    let (_lock, app) = fresh().await;
    for (method, script) in [
        ("GET", "get.php"),
        ("POST", "post.php"),
        ("PUT", "put.php"),
        ("DELETE", "delete.php"),
    ] {
        let r = send(&app, method, &format!("/api/{script}"), Some("{}")).await;
        assert_eq!(
            header(&r, "access-control-allow-methods"),
            method,
            "{script}"
        );
        assert_eq!(header(&r, "access-control-allow-origin"), "*", "{script}");
    }
}

// PHP: 200 for a missing id, and anything goes as an id: "16abc" is 16.
#[tokio::test]
async fn ids_are_checked() {
    let (_lock, app) = fresh().await;
    let cases = [
        (None, "Error: Student ID is missing!"),
        (Some("{}"), "Error: Student ID is missing!"),
        (Some(r#"{"id":null}"#), "Error: Student ID is missing!"),
        (
            Some(r#"{"id":"16abc"}"#),
            "Error: Student ID must be a positive whole number",
        ),
        (
            Some(r#"{"id":[15]}"#),
            "Error: Student ID must be a positive whole number",
        ),
        (
            Some(r#"{"id":15.9}"#),
            "Error: Student ID must be a positive whole number",
        ),
        (
            Some(r#"{"id":0}"#),
            "Error: Student ID must be a positive whole number",
        ),
    ];
    for (body, message) in cases {
        let r = send(&app, "GET", "/api/get_one.php", body).await;
        // A missing body is not a JSON object.
        let message = if body.is_none() {
            "Error: the body must be a JSON object"
        } else {
            message
        };
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{body:?}");
        assert_eq!(json(&r), json!({"message": message}), "{body:?}");
    }
    // A string of digits is still accepted, as clients sent them.
    let r = send(&app, "GET", "/api/get_one.php", Some(r#"{"id":"16"}"#)).await;
    assert_eq!(json(&r)["name"], "Joel");
}

// PHP: 200 `{"message":"No records found!"}`.
#[tokio::test]
async fn unknown_id_is_404() {
    let (_lock, app) = fresh().await;
    let r = send(&app, "GET", "/api/get_one.php", Some(r#"{"id":99}"#)).await;
    assert_eq!(r.status, StatusCode::NOT_FOUND);
    assert_eq!(json(&r), json!({"message": "No records found!"}));
}

// PHP: "Student updated" and "Student deleted" for an id that matches nothing.
#[tokio::test]
async fn update_and_delete_of_unknown_id_are_404() {
    let (_lock, app) = fresh().await;
    let before = students_tsv().await;
    let put = send(
        &app,
        "PUT",
        "/api/put.php",
        Some(r#"{"id":99,"name":"A","address":"B","age":1}"#),
    )
    .await;
    let delete = send(&app, "DELETE", "/api/delete.php", Some(r#"{"id":99}"#)).await;
    assert_eq!(put.status, StatusCode::NOT_FOUND);
    assert_eq!(delete.status, StatusCode::NOT_FOUND);
    assert_eq!(students_tsv().await, before);
    // Updating a row to the values it already has is not "not found".
    let same = send(
        &app,
        "PUT",
        "/api/put.php",
        Some(r#"{"id":16,"name":"Joel","address":"Delhi","age":23}"#),
    )
    .await;
    assert_eq!(same.status, StatusCode::OK);
}

// PHP: 200, `{"message":"Student added"}`, and no way to learn the new id.
#[tokio::test]
async fn post_is_201_with_the_new_id() {
    let (_lock, app) = fresh().await;
    let r = send(
        &app,
        "POST",
        "/api/post.php",
        Some(r#"{"name":"Asha","address":"Pune","age":21}"#),
    )
    .await;
    assert_eq!(r.status, StatusCode::CREATED);
    assert_eq!(json(&r), json!({"message": "Student added", "id": 18}));
}

// PHP: notices and an uncaught PDOException with a stack trace, as HTML, with 200.
#[tokio::test]
async fn missing_fields_are_422() {
    let (_lock, app) = fresh().await;
    let r = send(
        &app,
        "POST",
        "/api/post.php",
        Some(r#"{"name":"Lee","address":"Ooty"}"#),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(json(&r), json!({"message": "Error: age is missing!"}));
    let r = send(
        &app,
        "PUT",
        "/api/put.php",
        Some(r#"{"id":15,"name":"Dave"}"#),
    )
    .await;
    assert_eq!(json(&r), json!({"message": "Error: address is missing!"}));
}

// PHP: `true` stored as "1", 19.5 as 20, `["a"]` as "Array", "abc" a stack trace.
#[tokio::test]
async fn field_types_are_checked() {
    let (_lock, app) = fresh().await;
    let before = students_tsv().await;
    let cases = [
        (
            r#"{"name":true,"address":"x","age":1}"#,
            "Error: name must be a string",
        ),
        (
            r#"{"name":["a"],"address":"x","age":1}"#,
            "Error: name must be a string",
        ),
        (
            r#"{"name":"a","address":{},"age":1}"#,
            "Error: address must be a string",
        ),
        (
            r#"{"name":"","address":"x","age":1}"#,
            "Error: name must be 1 to 100 characters",
        ),
        (
            r#"{"name":"a","address":"x","age":"22"}"#,
            "Error: age must be a whole number",
        ),
        (
            r#"{"name":"a","address":"x","age":19.5}"#,
            "Error: age must be a whole number",
        ),
        (
            r#"{"name":"a","address":"x","age":1e20}"#,
            "Error: age must be a whole number",
        ),
    ];
    for (body, message) in cases {
        let r = send(&app, "POST", "/api/post.php", Some(body)).await;
        assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY, "{body}");
        assert_eq!(json(&r), json!({"message": message}), "{body}");
    }
    let long = "a".repeat(101);
    let r = send(
        &app,
        "POST",
        "/api/post.php",
        Some(&format!(r#"{{"name":"{long}","address":"x","age":1}}"#)),
    )
    .await;
    assert_eq!(r.status, StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(students_tsv().await, before);
}

// PHP: carried on with `null` and printed notices.
#[tokio::test]
async fn invalid_json_is_400() {
    let (_lock, app) = fresh().await;
    for body in ["not json", "[16]", "null"] {
        let r = send(&app, "POST", "/api/post.php", Some(body)).await;
        assert_eq!(r.status, StatusCode::BAD_REQUEST, "{body}");
        assert_eq!(
            json(&r),
            json!({"message": "Error: the body must be a JSON object"})
        );
    }
}

// PHP: a notice and a fatal error from the `$e` typo in config/Database.php.
#[tokio::test]
async fn database_down_is_503() {
    let app = students::router(students::App {
        pool: students::db::pool("mysql://root@127.0.0.1:1/studentdb").unwrap(),
        compat: false,
    });
    let r = send(&app, "GET", "/api/get.php", None).await;
    assert_eq!(r.status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(json(&r), json!({"message": "Error: database unavailable"}));
}

// Not a fix, a constraint: while PHP and Rust share the database, both must
// store the same bytes. A name written by either reads back the same in both.
#[tokio::test]
async fn both_modes_store_text_the_same_way() {
    let (_lock, fixed) = fresh().await;
    let compat = app(true);
    let body = r#"{"name":"Zoë 😀","address":"12/B","age":30}"#;
    send(&compat, "POST", "/api/post.php", Some(body)).await;
    send(&fixed, "POST", "/api/post.php", Some(body)).await;
    let table = students_tsv().await;
    let rows: Vec<&str> = table
        .lines()
        .filter(|l| l.starts_with("18\t") || l.starts_with("19\t"))
        .collect();
    assert_eq!(
        rows[0].split_once('\t').unwrap().1,
        rows[1].split_once('\t').unwrap().1
    );

    let r = send(&fixed, "GET", "/api/get_one.php", Some(r#"{"id":18}"#)).await;
    assert_eq!(json(&r)["name"], "Zoë 😀");
    // Plain UTF-8 out, rather than `ë` and `\/`. Same JSON either way.
    assert!(String::from_utf8_lossy(&r.body).contains("Zoë 😀"));
}
