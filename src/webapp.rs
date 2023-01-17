use hyper::Error;
use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use libgout::context::Context;
use thiserror::Error;

use crate::api;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Hyper error")]
    DefaultError(#[from] hyper::Error),
}

pub struct WebApp {
    pub context: Context,
    pub router: Router,
    pub addr: SocketAddr,
}

impl WebApp {
    pub fn new(context: Context) -> Self {
        let config = context.config.clone();
        let router = WebApp::router().nest("/api", WebApp::api_router());
        Self {
            context: context,
            router,
            addr: config.port,
        }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        axum::Server::bind(&self.addr)
            .serve(self.router.clone().into_make_service())
            .await?;

        Ok(())
    }

    fn router() -> Router {
        Router::new()
    }

    fn api_router() -> Router {
        let router = Router::new()
            .route("/", get(api::bar))
            .route("/food/:id", get(api::food));

        router
    }
}
