use flyaway::{db, router, App, Mode};

#[tokio::main]
async fn main() {
    let url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://flyaway:flyaway@127.0.0.1:3327/flyaway".into());
    let addr = std::env::var("LISTEN").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let mode = if std::env::var("FLYAWAY_COMPAT").is_ok_and(|v| v == "1") {
        Mode::compat()
    } else {
        let password = std::env::var("FLYAWAY_ADMIN_PASSWORD")
            .expect("FLYAWAY_ADMIN_PASSWORD is required outside compat mode");
        Mode::fixed(&password)
    };

    let pool = db::pool(&url).expect("DATABASE_URL is not a valid MySQL URL");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("cannot listen");
    eprintln!("FlyAway on {addr}, {} mode", mode.name());
    axum::serve(listener, router(App { pool, mode }))
        .await
        .unwrap();
}
