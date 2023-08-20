use std::net::SocketAddr;

use axum::{routing::get, Router};
use thiserror::Error;

use crate::api::Api;
use crate::context::Context;
use crate::frontend::index;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Hyper error")]
    DefaultError(#[from] hyper::Error),
}

pub struct WebApp {
    pub context: Context,
    pub addr: SocketAddr,
    pub router: Router,
}

impl WebApp {
    pub fn new(context: Context) -> Self {
        let config = context.config.clone();

        let router = Router::new()
            .nest("/", Self::browser_router())
            .nest("/api", Self::api_router())
            .with_state(context.clone());

        Self {
            context,
            addr: config.addr,
            router,
        }
    }

    fn api_router() -> Router<Context> {
        Router::new().route("/status", get(Api::status))
    }

    fn browser_router() -> Router<Context> {
        Router::new().route("/", get(index::index))
    }

    pub async fn run(&self) -> Result<(), AppError> {
        axum::Server::bind(&self.addr)
            .serve(self.router.clone().into_make_service())
            .await?;

        Ok(())
    }
}
