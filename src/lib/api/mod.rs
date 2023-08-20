use crate::context::Context;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Serialize;
use tracing::{info, instrument};

pub struct Api;

#[derive(Serialize)]
pub struct Status<'a> {
    status: &'a str,
}

impl Api {
    #[instrument]
    pub async fn status(State(_state): State<Context>) -> impl IntoResponse {
        info!("status called!");
        Json(Status { status: "ok" })
    }
}
