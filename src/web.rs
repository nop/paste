use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};

use crate::{
    error::Error,
    store::{ModelController, Store},
};

pub fn routes_paste(mc: ModelController) -> Router {
    Router::new()
        .route("/", post(create_paste))
        .route("/:id", get(get_paste))
        .with_state(mc)
}

#[axum::debug_handler]
pub async fn create_paste(
    State(mut store): State<ModelController>,
    content: String,
) -> impl IntoResponse {
    match store.create(content).await {
        Ok(id) => {
            tracing::debug!("created paste with id {}", id);
            (
                StatusCode::CREATED,
                format!("http://localhost:3000/{}\n", id),
            )
        }
        Err(Error::PasteExists(id)) => {
            tracing::debug!("paste already exists with id {}", id);
            (StatusCode::OK, format!("http://localhost:3000/{}\n", id))
        }
        Err(e) => {
            tracing::error!("paste creation error: {}", e);
            todo!("Handle paste creation error");
            // return e.to_string();
        }
    }
}

#[axum::debug_handler]
pub async fn get_paste(
    State(store): State<ModelController>,
    Path(id): Path<usize>,
) -> impl IntoResponse {
    store.read(&id).await.ok_or(Error::PasteNotFound)
}
