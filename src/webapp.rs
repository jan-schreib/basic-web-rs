use std::{net::SocketAddr, sync::Arc};

use axum::{routing::{get, post}, Router};
use libgout::{context::Context, db::interface::Db};
use thiserror::Error;

use crate::api;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Hyper error")]
    DefaultError(#[from] hyper::Error),
}

pub struct WebApp {
    pub context: Context,
    pub addr: SocketAddr,
}

impl WebApp {
    pub fn new(context: Context) -> Self {
        let config = context.config.clone();

        Self {
            context: context,
            addr: config.port,
        }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        let api_router = WebApp::api_router(self.context.database.connection.clone());
        let router = Router::new().nest("/api", api_router);

        axum::Server::bind(&self.addr)
            .serve(router.clone().into_make_service())
            .await?;

        Ok(())
    }

    fn api_router<S, T: Db + Send + Sync + Clone + 'static>(db: Arc<T>) -> Router<S> {
        let router = Router::new()
            .route("/foods/:id", get(api::food))
            .route("/foods", get(api::list_foods))
            .route("/food", post(api::create_food))
            .with_state(db);

        router
    }
}
