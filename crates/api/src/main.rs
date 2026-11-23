use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::{env, net::SocketAddr};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    service: &'static str,
}

async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: "sherlock-api",
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            env::var("RUST_LOG").unwrap_or_else(|_| "sherlock_api=info,tower_http=info".into()),
        )
        .init();

    let app = Router::new()
        .route("/healthz", get(health))
        .layer(TraceLayer::new_for_http());

    let address: SocketAddr = env::var("SHERLOCK_BIND")
        .unwrap_or_else(|_| "0.0.0.0:8080".into())
        .parse()?;

    let listener = TcpListener::bind(address).await?;
    tracing::info!(%address, "Sherlock API listening");
    axum::serve(listener, app).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_endpoint_returns_service_identity() {
        let response = health().await.0;
        assert_eq!(response.status, "ok");
        assert_eq!(response.service, "sherlock-api");
    }
}
