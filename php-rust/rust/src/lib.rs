//! A Rust port of php_rest_api_without_framework: the same five URLs on the
//! same `students` table.
//!
//! `STUDENTS_COMPAT=1` answers exactly as the PHP did (see `compat`). Without
//! it, the bugs are fixed (see `fixed`).

pub mod compat;
pub mod db;
pub mod fixed;
pub mod php;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{header, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::any;
use axum::Router;
use sqlx::MySqlPool;
use std::sync::Arc;

pub const SCRIPTS: [&str; 5] = [
    "get.php",
    "get_one.php",
    "post.php",
    "put.php",
    "delete.php",
];

#[derive(Clone)]
pub struct App {
    pub pool: MySqlPool,
    pub compat: bool,
}

/// One route per PHP script, at the path the script had. Each accepts every
/// method, because the scripts answered every method themselves.
pub fn router(app: App) -> Router {
    let app = Arc::new(app);
    SCRIPTS
        .into_iter()
        .fold(Router::new(), |router, script| {
            router.route(
                &format!("/api/{script}"),
                any(
                    move |State(app): State<Arc<App>>, method: Method, body: Bytes| async move {
                        handle(&app, script, &method, &body).await
                    },
                ),
            )
        })
        .with_state(app)
}

async fn handle(app: &App, script: &'static str, method: &Method, body: &[u8]) -> Response {
    if app.compat {
        let reply = compat::run(script, method, body, &app.pool).await;
        let mut response = (StatusCode::OK, reply.body).into_response();
        let headers = response.headers_mut();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_static("*"),
        );
        if reply.allow_methods_post {
            headers.insert(
                header::ACCESS_CONTROL_ALLOW_METHODS,
                HeaderValue::from_static("POST"),
            );
        }
        return response;
    }

    let reply = fixed::run(script, method, body, &app.pool).await;
    let allowed = HeaderValue::from_str(fixed::method_for(script).as_str()).unwrap();
    let mut response = (reply.status, axum::Json(reply.body)).into_response();
    let headers = response.headers_mut();
    headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        HeaderValue::from_static("*"),
    );
    headers.insert(header::ACCESS_CONTROL_ALLOW_METHODS, allowed.clone());
    if reply.status == StatusCode::METHOD_NOT_ALLOWED {
        headers.insert(header::ALLOW, allowed);
    }
    response
}
