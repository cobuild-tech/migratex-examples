//! The five scripts in `api/`, reproduced exactly: same bodies, same headers,
//! HTTP 200 every time, PHP's notices and fatal errors included.
//!
//! Each function follows its script line by line. The numbers passed to
//! `prop`, `bind` and the error helpers are the line numbers PHP prints.

use crate::db;
use crate::php::{self, Halt, Output};
use axum::http::Method;
use serde_json::Value;
use sqlx::MySqlPool;

pub struct Reply {
    pub body: Vec<u8>,
    /// `header('Access-Control-Allow-Methods: POST')`, which post.php,
    /// put.php and delete.php all send.
    pub allow_methods_post: bool,
}

pub async fn run(script: &'static str, method: &Method, body: &[u8], pool: &MySqlPool) -> Reply {
    let mut out = Output::new(script);
    let _ = match script {
        "get.php" => get(&mut out, method, pool).await,
        "get_one.php" => get_one(&mut out, method, body, pool).await,
        "post.php" => post(&mut out, method, body, pool).await,
        "put.php" => put(&mut out, method, body, pool).await,
        "delete.php" => delete(&mut out, method, body, pool).await,
        _ => unreachable!("no route for {script}"),
    };
    Reply {
        body: out.body,
        allow_methods_post: matches!(script, "post.php" | "put.php" | "delete.php"),
    }
}

/// `$db = (new Database())->connect();`
async fn connect(out: &mut Output, pool: &MySqlPool, line: u32) -> Result<db::Conn, Halt> {
    pool.acquire()
        .await
        .map_err(|_| out.database_connect_failed(line))
}

/// `echo json_encode($row)` for one student. PDO returned every column as a
/// string, so `id` and `age` are strings in the JSON.
fn student_json(s: &db::Student) -> Option<String> {
    let id = s.id.to_string();
    let age = s.age.to_string();
    php::json_encode_object(&[
        ("id", id.as_bytes()),
        ("name", &s.name),
        ("address", &s.address),
        ("age", age.as_bytes()),
    ])
}

async fn get(out: &mut Output, method: &Method, pool: &MySqlPool) -> Result<(), Halt> {
    if method != Method::GET {
        out.message("Error: incorrect Method!");
        return Ok(());
    }
    let mut conn = connect(out, pool, 11).await?;
    let students = db::fetch_all(&mut conn).await.unwrap_or_default();
    if students.is_empty() {
        // An empty table is not `[]` but this object.
        out.message("No records found!");
        return Ok(());
    }
    let items: Option<Vec<String>> = students.iter().map(student_json).collect();
    // json_encode returns false for invalid UTF-8, and `echo false` prints nothing.
    if let Some(items) = items {
        out.echo(format!("[{}]", items.join(",")).as_bytes());
    }
    Ok(())
}

async fn get_one(
    out: &mut Output,
    method: &Method,
    body: &[u8],
    pool: &MySqlPool,
) -> Result<(), Halt> {
    if method != Method::GET {
        out.message("Error: incorrect Method!");
        return Ok(());
    }
    let mut conn = connect(out, pool, 11).await?;
    // The id comes from a JSON body, on a GET request.
    let data = php::json_decode(body);
    if !php::isset(&data, "id") {
        out.message("Error: Student ID is missing!");
        return Ok(());
    }
    let id = out.bind(data.as_ref().and_then(|d| d.get("id")), 26)?;
    match db::fetch_one(&mut conn, id.as_deref()).await.ok().flatten() {
        // print_r(json_encode(...)) prints the JSON, or nothing for false.
        Some(student) => out.echo(student_json(&student).unwrap_or_default().as_bytes()),
        None => out.message("No records found!"),
    }
    Ok(())
}

async fn post(
    out: &mut Output,
    method: &Method,
    body: &[u8],
    pool: &MySqlPool,
) -> Result<(), Halt> {
    if method != Method::POST {
        out.message("Error: incorrect Method!");
        return Ok(());
    }
    let mut conn = connect(out, pool, 12).await?;
    let data = php::json_decode(body);
    let name = out.prop(&data, "name", 18);
    let address = out.prop(&data, "address", 19);
    let age = out.prop(&data, "age", 20);

    // Student::postData()
    let name = out.bind(name, 49)?;
    let address = out.bind(address, 50)?;
    let age = out.bind(age, 51)?;
    db::insert(
        &mut conn,
        name.as_deref(),
        address.as_deref(),
        age.as_deref(),
    )
    .await
    .map_err(|e| out.uncaught_pdo_exception(&db::pdo_message(&e), 53, "postData", 22))?;
    out.message("Student added");
    Ok(())
}

async fn put(out: &mut Output, method: &Method, body: &[u8], pool: &MySqlPool) -> Result<(), Halt> {
    if method != Method::PUT {
        out.message("Error: incorrect Method!");
        return Ok(());
    }
    let mut conn = connect(out, pool, 12).await?;
    let data = php::json_decode(body);
    let id = data
        .as_ref()
        .and_then(|d| d.get("id"))
        .filter(|_| php::isset(&data, "id"));
    // Read before the id is checked, so a missing id still prints these notices.
    let name = out.prop(&data, "name", 19);
    let address = out.prop(&data, "address", 20);
    let age = out.prop(&data, "age", 21);
    if id.is_none() {
        out.message("Error: Student ID is missing!");
        return Ok(());
    }

    // Student::putData()
    let name = out.bind(name, 64)?;
    let address = out.bind(address, 65)?;
    let age = out.bind(age, 66)?;
    let id = out.bind(id, 67)?;
    db::update(
        &mut conn,
        id.as_deref(),
        name.as_deref(),
        address.as_deref(),
        age.as_deref(),
    )
    .await
    .map_err(|e| out.uncaught_pdo_exception(&db::pdo_message(&e), 69, "putData", 25))?;
    // Also when no row has this id.
    out.message("Student updated");
    Ok(())
}

async fn delete(
    out: &mut Output,
    method: &Method,
    body: &[u8],
    pool: &MySqlPool,
) -> Result<(), Halt> {
    if method != Method::DELETE {
        out.message("Error: incorrect Method!");
        return Ok(());
    }
    let mut conn = connect(out, pool, 12).await?;
    let data = php::json_decode(body);
    let id: Option<&Value> = data
        .as_ref()
        .and_then(|d| d.get("id"))
        .filter(|_| php::isset(&data, "id"));
    if id.is_none() {
        out.message("Error: Student ID is missing!");
        return Ok(());
    }

    // Student::delete()
    let id = out.bind(id, 79)?;
    db::delete(&mut conn, id.as_deref())
        .await
        .map_err(|e| out.uncaught_pdo_exception(&db::pdo_message(&e), 81, "delete", 22))?;
    // Also when no row has this id.
    out.message("Student deleted");
    Ok(())
}
