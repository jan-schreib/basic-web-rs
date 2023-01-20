use std::sync::Arc;

use crate::{
    db::{error::DbError, interface::Db},
    types::food::FoodInsert,
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
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

pub async fn food(Path(food_id): Path<u64>) -> impl IntoResponse {
    Json(food_id)
}

pub async fn list_foods<T: Db>(State(db): State<Arc<T>>) -> Result<impl IntoResponse, ApiError> {
    let foods = db.get_foods().await?;

    Ok(Json(foods))
}

pub async fn create_food<T: Db>(
    State(db): State<Arc<T>>,
    Json(params): Json<FoodInsert>,
) -> Result<impl IntoResponse, ApiError> {
    db.add_food(&params).await?;
    Ok(Json("Ok"))
}
