use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use libgout::{db::interface::Db, types::food::FoodInsert};

pub async fn food(Path(food_id): Path<u64>) -> impl IntoResponse {
    Json(food_id)
}

pub async fn list_foods<T: Db>(State(db): State<Arc<T>>) -> impl IntoResponse {
    let foods = db.get_foods().await.expect("DEAD");

    Json(foods)
}

pub async fn create_food<T: Db>(State(db): State<Arc<T>>, Json(params): Json<FoodInsert>) -> impl IntoResponse {
    db.add_food(&params).await.expect("UDOM");
    Json("Ok")
}
