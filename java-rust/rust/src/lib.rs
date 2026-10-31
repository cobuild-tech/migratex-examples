//! A Rust port of FlyAway, a JSP and Servlet flight-booking app, on the same
//! URLs and the same `flight` table.
//!
//! `FLYAWAY_COMPAT=1` answers exactly as the Java did on Tomcat 9.0.44 (see
//! `compat`). Without it, the bugs are fixed (see `fixed`).

pub mod compat;
pub mod db;
pub mod fixed;
pub mod java;
pub mod jsp;
pub mod tomcat;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderName, HeaderValue, Method, Uri};
use axum::response::{IntoResponse, Response};
use axum::Router;
use sqlx::MySqlPool;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum Mode {
    /// The Java's static fields. The lock is the one thing the port adds:
    /// Java let two requests write the fields at once.
    Compat(Arc<Mutex<compat::Statics>>),
    Fixed(fixed::Admin),
}

impl Mode {
    pub fn compat() -> Mode {
        Mode::Compat(Arc::default())
    }

    pub fn fixed(admin_password: &str) -> Mode {
        Mode::Fixed(fixed::Admin::new(admin_password))
    }

    pub fn name(&self) -> &'static str {
        match self {
            Mode::Compat(_) => "compat",
            Mode::Fixed(_) => "fixed",
        }
    }
}

#[derive(Clone)]
pub struct App {
    pub pool: MySqlPool,
    pub mode: Mode,
}

pub fn router(app: App) -> Router {
    match app.mode {
        Mode::Compat(statics) => Router::new()
            .fallback(compat_handler)
            .with_state((app.pool, statics)),
        Mode::Fixed(admin) => fixed::router(app.pool, admin),
    }
}

type CompatState = (MySqlPool, Arc<Mutex<compat::Statics>>);

/// Every request goes through the one handler, which routes it as Tomcat
/// would, because Tomcat's answers to odd methods and paths are part of
/// what clients saw.
async fn compat_handler(
    State((pool, statics)): State<CompatState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    // Tomcat reads a form body only on a POST with the form content type.
    let is_form = headers
        .get(header::CONTENT_TYPE)
        .and_then(|v| v.to_str().ok())
        .is_some_and(|ct| ct.starts_with("application/x-www-form-urlencoded"));
    let form = (method == Method::POST && is_form).then_some(&body[..]);
    let params = tomcat::Params::parse(uri.query(), form);

    let mut statics = statics.lock().await;
    let reply = compat::handle(&mut statics, &pool, &method, uri.path(), &params).await;
    into_response(reply)
}

pub fn into_response(reply: tomcat::Reply) -> Response {
    let mut response = (reply.status, reply.body).into_response();
    let headers = response.headers_mut();
    // `Vec<u8>` brings a content type of its own; Tomcat's are set below.
    headers.remove(header::CONTENT_TYPE);
    for (name, value) in reply.headers {
        headers.insert(
            HeaderName::from_static(name),
            HeaderValue::from_str(&value).unwrap(),
        );
    }
    response
}
