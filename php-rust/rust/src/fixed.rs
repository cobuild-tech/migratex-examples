//! The same API with its bugs fixed. This is the default mode.
//!
//! Same URLs, same messages, same database. What changes is listed in the
//! README under "Fixed in the default mode", and each change has a test in
//! `tests/fixed.rs`.

use crate::db;
use axum::http::{Method, StatusCode};
use serde_json::{json, Map, Value};
use sqlx::MySqlPool;

pub struct Reply {
    pub status: StatusCode,
    pub body: Value,
}

fn reply(status: StatusCode, message: &str) -> Reply {
    Reply {
        status,
        body: json!({ "message": message }),
    }
}

/// The method each script accepts. PHP sent `Allow-Methods: POST` for all
/// three writes, and nothing for the reads.
pub fn method_for(script: &str) -> Method {
    match script {
        "post.php" => Method::POST,
        "put.php" => Method::PUT,
        "delete.php" => Method::DELETE,
        _ => Method::GET,
    }
}

pub async fn run(script: &str, method: &Method, body: &[u8], pool: &MySqlPool) -> Reply {
    if *method != method_for(script) {
        return reply(StatusCode::METHOD_NOT_ALLOWED, "Error: incorrect Method!");
    }
    let mut conn = match pool.acquire().await {
        Ok(conn) => conn,
        // PHP: a notice and a fatal error from the `$e` typo in Database.php.
        Err(_) => {
            return reply(
                StatusCode::SERVICE_UNAVAILABLE,
                "Error: database unavailable",
            )
        }
    };
    let result = match script {
        "get.php" => get(&mut conn).await,
        "get_one.php" => get_one(&mut conn, body).await,
        "post.php" => post(&mut conn, body).await,
        "put.php" => put(&mut conn, body).await,
        "delete.php" => delete(&mut conn, body).await,
        _ => unreachable!("no route for {script}"),
    };
    result.unwrap_or_else(|err| {
        if db::is_bad_value(&err) {
            return reply(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Error: a value is too long or out of range",
            );
        }
        // PHP: the exception and a stack trace, in the response body.
        eprintln!("database error in {script}: {err}");
        reply(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Error: something went wrong, try again!",
        )
    })
}

/// Numbers as JSON numbers. PHP sent `"id":"15"` because PDO returns strings.
fn student_json(s: &db::Student) -> Value {
    json!({
        "id": s.id,
        "name": String::from_utf8_lossy(&s.name),
        "address": String::from_utf8_lossy(&s.address),
        "age": s.age,
    })
}

async fn get(conn: &mut db::Conn) -> Result<Reply, sqlx::Error> {
    let students = db::fetch_all(conn).await?;
    // PHP: `{"message":"No records found!"}` for an empty table.
    let list: Vec<Value> = students.iter().map(student_json).collect();
    Ok(Reply {
        status: StatusCode::OK,
        body: Value::Array(list),
    })
}

/// The body as a JSON object, or the 400 to send. PHP read any body, valid or
/// not, and carried on with `null`.
fn object(body: &[u8]) -> Result<Map<String, Value>, Reply> {
    match serde_json::from_slice(body) {
        Ok(Value::Object(map)) => Ok(map),
        _ => Err(reply(
            StatusCode::BAD_REQUEST,
            "Error: the body must be a JSON object",
        )),
    }
}

/// An id is a positive whole number, or a string of digits, which is what
/// clients of the PHP version sent. PHP took anything: `"16abc"` meant 16 and
/// `[15]` meant `"Array"`.
fn id(data: &Map<String, Value>) -> Result<u32, Reply> {
    let missing = || reply(StatusCode::BAD_REQUEST, "Error: Student ID is missing!");
    let invalid = || {
        reply(
            StatusCode::BAD_REQUEST,
            "Error: Student ID must be a positive whole number",
        )
    };
    match data.get("id") {
        None | Some(Value::Null) => Err(missing()),
        Some(Value::Number(n)) => n
            .as_u64()
            .and_then(|n| u32::try_from(n).ok())
            .filter(|&n| n > 0)
            .ok_or_else(invalid),
        Some(Value::String(s)) if !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) => {
            s.parse().ok().filter(|&n| n > 0).ok_or_else(invalid)
        }
        Some(_) => Err(invalid()),
    }
}

struct Fields {
    name: String,
    address: String,
    age: i32,
}

/// name and address are non-empty strings of at most 100 characters, and age
/// is a whole number. PHP stored `true` as `"1"`, `19.5` as 20, `[]` as
/// `"Array"`, and failed with a stack trace when a field was missing.
fn fields(data: &Map<String, Value>) -> Result<Fields, Reply> {
    let text = |key: &str| match data.get(key) {
        Some(Value::String(s)) if !s.trim().is_empty() && s.chars().count() <= 100 => Ok(s.clone()),
        Some(Value::String(_)) => Err(reply(
            StatusCode::UNPROCESSABLE_ENTITY,
            &format!("Error: {key} must be 1 to 100 characters"),
        )),
        None | Some(Value::Null) => Err(reply(
            StatusCode::UNPROCESSABLE_ENTITY,
            &format!("Error: {key} is missing!"),
        )),
        Some(_) => Err(reply(
            StatusCode::UNPROCESSABLE_ENTITY,
            &format!("Error: {key} must be a string"),
        )),
    };
    let name = text("name")?;
    let address = text("address")?;
    let age = match data.get("age") {
        Some(Value::Number(n)) => {
            n.as_i64()
                .and_then(|n| i32::try_from(n).ok())
                .ok_or_else(|| {
                    reply(
                        StatusCode::UNPROCESSABLE_ENTITY,
                        "Error: age must be a whole number",
                    )
                })?
        }
        None | Some(Value::Null) => {
            return Err(reply(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Error: age is missing!",
            ))
        }
        Some(_) => {
            return Err(reply(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Error: age must be a whole number",
            ))
        }
    };
    Ok(Fields { name, address, age })
}

async fn get_one(conn: &mut db::Conn, body: &[u8]) -> Result<Reply, sqlx::Error> {
    let id = match object(body).and_then(|data| id(&data)) {
        Ok(id) => id,
        Err(reply) => return Ok(reply),
    };
    Ok(match db::fetch_one(conn, Some(&id.to_string())).await? {
        Some(student) => Reply {
            status: StatusCode::OK,
            body: student_json(&student),
        },
        None => reply(StatusCode::NOT_FOUND, "No records found!"),
    })
}

async fn post(conn: &mut db::Conn, body: &[u8]) -> Result<Reply, sqlx::Error> {
    let f = match object(body).and_then(|data| fields(&data)) {
        Ok(f) => f,
        Err(reply) => return Ok(reply),
    };
    let id = db::insert(
        conn,
        Some(&f.name),
        Some(&f.address),
        Some(&f.age.to_string()),
    )
    .await?;
    // PHP: 200, and no way to learn the new id.
    Ok(Reply {
        status: StatusCode::CREATED,
        body: json!({ "message": "Student added", "id": id }),
    })
}

async fn put(conn: &mut db::Conn, body: &[u8]) -> Result<Reply, sqlx::Error> {
    let (id, f) = match object(body).and_then(|data| Ok((id(&data)?, fields(&data)?))) {
        Ok(v) => v,
        Err(reply) => return Ok(reply),
    };
    // PHP: "Student updated" for an id that matches nothing.
    if !db::exists(conn, id).await? {
        return Ok(reply(StatusCode::NOT_FOUND, "No records found!"));
    }
    let id = id.to_string();
    db::update(
        conn,
        Some(&id),
        Some(&f.name),
        Some(&f.address),
        Some(&f.age.to_string()),
    )
    .await?;
    Ok(reply(StatusCode::OK, "Student updated"))
}

async fn delete(conn: &mut db::Conn, body: &[u8]) -> Result<Reply, sqlx::Error> {
    let id = match object(body).and_then(|data| id(&data)) {
        Ok(id) => id,
        Err(reply) => return Ok(reply),
    };
    // PHP: "Student deleted" for an id that matches nothing.
    if !db::exists(conn, id).await? {
        return Ok(reply(StatusCode::NOT_FOUND, "No records found!"));
    }
    db::delete(conn, Some(&id.to_string())).await?;
    Ok(reply(StatusCode::OK, "Student deleted"))
}
