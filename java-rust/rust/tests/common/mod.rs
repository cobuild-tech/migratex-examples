//! Shared by the test files: an app and a way to send it a request.

#![allow(dead_code)]

use axum::body::Body;
use axum::http::{header, HeaderMap, Method, Request, StatusCode};
use flyaway::{db, router, App, Mode};
use http_body_util::BodyExt;
use std::path::PathBuf;
use tower::ServiceExt;

pub fn database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://flyaway:flyaway@127.0.0.1:3327/flyaway".into())
}

pub fn java_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../java")
}

/// A fresh app: in compat mode, a freshly started Tomcat.
pub fn app(mode: Mode) -> axum::Router {
    router(App {
        pool: db::pool(&database_url()).unwrap(),
        mode,
    })
}

pub struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

impl Response {
    pub fn text(&self) -> String {
        String::from_utf8_lossy(&self.body).into_owned()
    }

    pub fn header(&self, name: &str) -> Option<&str> {
        self.headers.get(name).and_then(|v| v.to_str().ok())
    }
}

/// One request, as capture-golden.sh sends it: a BODY is a form post.
/// `cookie` carries a session in the fixed-mode tests.
pub async fn send_with(
    app: &axum::Router,
    method: &str,
    path: &str,
    body: Option<&str>,
    cookie: Option<&str>,
) -> Response {
    let mut request = Request::builder()
        .method(Method::from_bytes(method.as_bytes()).unwrap())
        .uri(path);
    if body.is_some() {
        request = request.header(header::CONTENT_TYPE, "application/x-www-form-urlencoded");
    }
    if let Some(cookie) = cookie {
        request = request.header(header::COOKIE, cookie);
    }
    let request = request
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

pub async fn send(app: &axum::Router, method: &str, path: &str, body: Option<&str>) -> Response {
    send_with(app, method, path, body, None).await
}
