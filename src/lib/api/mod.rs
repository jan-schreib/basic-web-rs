use crate::context::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;

pub struct Api;

#[derive(Serialize)]
pub struct Status<'a> {
    status: &'a str,
}

impl Api {
    pub async fn status(State(_state): State<Context>) -> impl IntoResponse {
        Json(Status { status: "ok" })
    }
}
