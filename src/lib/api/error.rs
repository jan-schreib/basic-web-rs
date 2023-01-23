use crate::db::error::DbError;
use axum::response::IntoResponse;
use hyper::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("DbError")]
    DbError(#[from] DbError),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiError::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
        }
        .into_response()
    }
}
