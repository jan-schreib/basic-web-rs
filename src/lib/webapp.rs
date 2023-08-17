use std::{net::SocketAddr, sync::Arc};

use axum::Router;
use thiserror::Error;

use crate::{context::Context, db::interface::Db};

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

        let api_router = WebApp::api_router(context.database.connection.clone());
        let router = Router::new().nest("/api", api_router);

        Self {
            context,
            addr: config.addr,
            router,
        }
    }

    pub async fn run(&self) -> Result<(), AppError> {
        axum::Server::bind(&self.addr)
            .serve(self.router.clone().into_make_service())
            .await?;

        Ok(())
    }

    fn api_router<S, T: Db + Send + Sync + Clone + 'static>(db: Arc<T>) -> Router<S> {
        let api = Router::new();

        Router::new().merge(api).with_state(db)
    }
}
