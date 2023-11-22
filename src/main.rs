use std::net::SocketAddr;

use axum::Router;
use tower_http::services::ServeFile;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "paste=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let routes_all = Router::new().merge(routes_static());

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(routes_all.into_make_service())
        .await
        .unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/", ServeFile::new("./paste.1.txt"))
}
