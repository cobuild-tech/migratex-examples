//! The `flight` table. FlyAway only ever reads it.

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{MySqlPool, Row};
use std::str::FromStr;
use std::time::Duration;

/// A row, as `ResultSet.getString` gives it: every column is text, and any
/// but the key may be NULL.
#[derive(Debug, Clone)]
pub struct Flight {
    pub flight_id: String,
    pub name: Option<String>,
    pub source: Option<String>,
    pub destination: Option<String>,
    pub days: Option<String>,
    pub ticket_price: Option<String>,
}

/// Connects lazily, as the JSPs did on each request, so the service starts
/// without the database and a page fails the way the JSP's did.
pub fn pool(url: &str) -> Result<MySqlPool, sqlx::Error> {
    let options = MySqlConnectOptions::from_str(url)?;
    Ok(MySqlPoolOptions::new()
        .acquire_timeout(Duration::from_secs(5))
        .connect_lazy_with(options))
}

fn flight(row: &sqlx::mysql::MySqlRow) -> Result<Flight, sqlx::Error> {
    Ok(Flight {
        flight_id: row.try_get("flight_id")?,
        name: row.try_get("name")?,
        source: row.try_get("source")?,
        destination: row.try_get("destination")?,
        days: row.try_get("days")?,
        ticket_price: row.try_get("ticket_price")?,
    })
}

/// Runs SQL built by string concatenation, as `Search.getQuery()` does.
/// Compat mode only. A prepared statement takes one statement, as
/// Connector/J's default `allowMultiQueries=false` does, so injected text can
/// change what is read but can't add a second statement.
pub async fn flights_by_query(pool: &MySqlPool, sql: &str) -> Result<Vec<Flight>, sqlx::Error> {
    let rows = sqlx::query(sql).fetch_all(pool).await.inspect_err(|e| {
        eprintln!("search-results.jsp: {e}");
    })?;
    rows.iter().map(flight).collect()
}

pub async fn all_flights(pool: &MySqlPool) -> Result<Vec<Flight>, sqlx::Error> {
    let rows = sqlx::query("SELECT * FROM flight").fetch_all(pool).await?;
    rows.iter().map(flight).collect()
}

/// The fixed mode's search: the same match, with the values bound.
pub async fn search(
    pool: &MySqlPool,
    source: &str,
    destination: &str,
    day: &str,
) -> Result<Vec<Flight>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT * FROM flight WHERE source = ? AND destination = ? AND days LIKE CONCAT('%', ?, '%')",
    )
    .bind(source)
    .bind(destination)
    .bind(day)
    .fetch_all(pool)
    .await?;
    rows.iter().map(flight).collect()
}

pub async fn flight_by_id(pool: &MySqlPool, id: &str) -> Result<Option<Flight>, sqlx::Error> {
    let row = sqlx::query("SELECT * FROM flight WHERE flight_id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await?;
    row.as_ref().map(flight).transpose()
}
