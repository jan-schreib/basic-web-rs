use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};
use libgout::db::interface::Db;

pub async fn bar() -> &'static str {
    "Bar!"
}

pub async fn food(Path(food_id): Path<u64>) -> impl IntoResponse {
    Json(food_id)
}

pub async fn list_foods<T: Db>(State(db): State<Arc<T>>) -> impl IntoResponse {
    let foods = db.get_foods().await.expect("DEAD");

    Json(foods)
}
