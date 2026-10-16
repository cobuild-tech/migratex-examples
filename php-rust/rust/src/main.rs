use students::{db, router, App};

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root@127.0.0.1:3317/studentdb".into());
    let compat = std::env::var("STUDENTS_COMPAT").is_ok_and(|v| v == "1");
    let addr = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".into());

    let pool = db::pool(&url).expect("DATABASE_URL is not a valid MySQL URL");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("cannot listen");
    let mode = if compat { "compat" } else { "fixed" };
    eprintln!("students API on {addr}, {mode} mode");
    axum::serve(listener, router(App { pool, compat }))
        .await
        .unwrap();
}
