use axum::{extract::Path, response::IntoResponse, Json};

pub async fn bar() -> &'static str {
    "Bar!"
}

pub async fn food(Path(food_id): Path<u64>) -> impl IntoResponse {
    Json(food_id)
}
