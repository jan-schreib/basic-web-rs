use std::{sync::Arc, collections::HashMap};

use crate::{
    db::interface::Db,
    types::food::{Food, FoodInsert},
};
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;

use super::error::ApiError;

// POST
pub async fn create_food<T: Db>(
    State(db): State<Arc<T>>,
    Json(params): Json<FoodInsert>,
) -> Result<impl IntoResponse, ApiError> {
    let created_id = db.add_food(&params).await?;
    let mut map = HashMap::new();
    map.insert("success", created_id);

    Ok((StatusCode::CREATED, Json(map)))
}

// GET
pub async fn get_food<T: Db>(
    State(db): State<Arc<T>>,
    food_id: Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    let food = db.get_food(*food_id).await?;

    Ok(Json(food))
}

// GET
pub async fn list_foods<T: Db>(State(db): State<Arc<T>>) -> Result<impl IntoResponse, ApiError> {
    let foods = db.list_food().await?;

    Ok(Json(foods))
}

// PATCH/PUT - only used as PUT
pub async fn update_food<T: Db>(
    State(db): State<Arc<T>>,
    Json(payload): Json<Food>,
) -> Result<impl IntoResponse, ApiError> {
    db.update_food(payload).await?;

    Ok(Json("Ok"))
}

// DELETE
pub async fn delete_food<T: Db>(
    State(db): State<Arc<T>>,
    food_id: Path<i64>,
) -> Result<impl IntoResponse, ApiError> {
    db.delete_food(*food_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
