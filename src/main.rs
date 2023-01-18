use std::net::SocketAddr;

use libgout::context::Context;
use libgout::types::config::Config;

use thiserror::Error;

mod api;
mod webapp;

use webapp::WebApp;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DbError")]
    DbError(#[from] libgout::db::error::DbError),
    #[error("ContextError")]
    ContextError(#[from] libgout::context::Error),
    #[error("WebAppError")]
    WebAppError(#[from] webapp::AppError),
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    let config = Config {
        db_url: "sqlite::memory:".to_string(),
        cache_url: String::new(),
        port: SocketAddr::from(([127, 0, 0, 1], 3000)),
    };

    let context = Context::new(&config).await?;
    context.run_migrations().await?;

    let app = WebApp::new(context);
    app.run().await?;

    Ok(())
}
