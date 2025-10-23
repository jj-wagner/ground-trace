use axum::{routing::get, Router};
use tokio::net::TcpListener;

async fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    // Simple logging
    tracing_subscriber::fmt::init();

    // Build the app
    let app = Router::new()
        .route("/healthz", get(healthz));

    // Allow override via env; default 0.0.0.0:8080
    let addr = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string());

    let listener = TcpListener::bind(&addr).await?;
    tracing::info!("listening on http://{addr}");

    axum::serve(listener, app).await?;
    Ok(())
}
