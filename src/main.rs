pub mod error;
mod store;
mod web;

use std::net::SocketAddr;

use axum::{routing::get_service, Router};
use store::ModelController;

use tower_http::services::ServeFile;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> crate::error::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "paste=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let state = ModelController::new().await?;

    let routes_all = Router::new()
        .route("/", get_service(ServeFile::new("./paste.1.txt")))
        .merge(web::routes_paste(state));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
    Ok(())
}
