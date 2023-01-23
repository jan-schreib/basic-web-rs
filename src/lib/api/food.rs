use std::sync::Arc;

use crate::{db::interface::Db, types::food::FoodInsert};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use super::error::ApiError;

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
