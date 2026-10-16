//! `models/Student.php`: the same five statements, with sqlx.
//!
//! Every value is bound as a string or `NULL`, as PDO's emulated prepares did,
//! so MariaDB applies the same conversions: `"16abc"` matches id 16, `"19.5"`
//! is stored as 20, and `"abc"` is rejected for `age`.

use sqlx::mysql::{MySqlConnectOptions, MySqlConnection, MySqlPool, MySqlPoolOptions};
use sqlx::Row;
use std::str::FromStr;
use std::time::Duration;

pub struct Student {
    pub id: u32,
    pub name: Vec<u8>,
    pub address: Vec<u8>,
    pub age: i32,
}

/// The PHP side connects without a charset, so the connection is `latin1`
/// while the columns are `utf8`. Every UTF-8 byte PHP sends is stored as one
/// latin1 character: `é` is stored as `Ã©`. Rust has to connect the same way
/// or the two would read each other's rows differently.
pub fn pool(url: &str) -> Result<MySqlPool, sqlx::Error> {
    let options = MySqlConnectOptions::from_str(url)?
        .charset("latin1")
        .collation("latin1_swedish_ci")
        // sqlx adds these to every session by default. PHP doesn't.
        .pipes_as_concat(false)
        .no_engine_substitution(false)
        .timezone(None);
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(3))
        .connect_lazy_with(options))
}

pub type Conn = sqlx::pool::PoolConnection<sqlx::MySql>;

fn student(row: sqlx::mysql::MySqlRow) -> Result<Student, sqlx::Error> {
    Ok(Student {
        id: row.try_get("id")?,
        name: row.try_get("name")?,
        address: row.try_get("address")?,
        age: row.try_get("age")?,
    })
}

pub async fn fetch_all(conn: &mut MySqlConnection) -> Result<Vec<Student>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM students")
        .fetch_all(conn)
        .await?;
    rows.into_iter().map(student).collect()
}

pub async fn fetch_one(
    conn: &mut MySqlConnection,
    id: Option<&str>,
) -> Result<Option<Student>, sqlx::Error> {
    let row = sqlx::query("SELECT  * FROM students WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    row.map(student).transpose()
}

/// Returns the new id.
pub async fn insert(
    conn: &mut MySqlConnection,
    name: Option<&str>,
    address: Option<&str>,
    age: Option<&str>,
) -> Result<u64, sqlx::Error> {
    let done = sqlx::query("INSERT INTO students SET name = ?, address = ?, age = ?")
        .bind(name)
        .bind(address)
        .bind(age)
        .execute(conn)
        .await?;
    Ok(done.last_insert_id())
}

pub async fn update(
    conn: &mut MySqlConnection,
    id: Option<&str>,
    name: Option<&str>,
    address: Option<&str>,
    age: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE students SET name = ?, address = ?, age = ? WHERE id = ?")
        .bind(name)
        .bind(address)
        .bind(age)
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

pub async fn delete(conn: &mut MySqlConnection, id: Option<&str>) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM students WHERE id = ?")
        .bind(id)
        .execute(conn)
        .await?;
    Ok(())
}

/// Whether a row with this id exists. Only the fixed mode asks: PHP reports
/// success for an id that matches nothing.
pub async fn exists(conn: &mut MySqlConnection, id: u32) -> Result<bool, sqlx::Error> {
    let row = sqlx::query("SELECT 1 FROM students WHERE id = ?")
        .bind(id)
        .fetch_optional(conn)
        .await?;
    Ok(row.is_some())
}

/// The text of the `PDOException` PDO would throw for this error, e.g.
/// `SQLSTATE[23000]: Integrity constraint violation: 1048 Column 'age' cannot be null`.
pub fn pdo_message(err: &sqlx::Error) -> String {
    if let Some(db) = err.as_database_error() {
        let mysql = db.downcast_ref::<sqlx::mysql::MySqlDatabaseError>();
        let state = mysql.code().unwrap_or("HY000");
        return format!(
            "SQLSTATE[{state}]: {}: {} {}",
            sqlstate_text(state),
            mysql.number(),
            mysql.message()
        );
    }
    format!("SQLSTATE[HY000]: General error: {err}")
}

/// The descriptions PDO prints for SQLSTATE classes (ext/pdo/pdo_sqlstate.c),
/// for the ones an INSERT or UPDATE on this table can raise.
fn sqlstate_text(state: &str) -> &'static str {
    match state {
        "01000" => "Warning",
        "21S01" => "Insert value list does not match column list",
        "22001" => "String data, right truncated",
        "22003" => "Numeric value out of range",
        "22007" => "Invalid datetime format",
        "22012" => "Division by zero",
        "23000" => "Integrity constraint violation",
        "42000" => "Syntax error or access violation",
        "42S02" => "Base table or view not found",
        "42S22" => "Column not found",
        _ => "General error",
    }
}

/// Error numbers the fixed mode turns into a 422 rather than a 500.
pub fn is_bad_value(err: &sqlx::Error) -> bool {
    err.as_database_error()
        .map(|db| {
            db.downcast_ref::<sqlx::mysql::MySqlDatabaseError>()
                .number()
        })
        .is_some_and(|n| matches!(n, 1048 | 1264 | 1366 | 1406))
}
