use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    PasteExists(usize),
    PasteNotFound,
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("error: {:?}", self);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Internal Server Error",
        )
            .into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for Error {}
