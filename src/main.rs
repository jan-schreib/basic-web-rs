use libbasicweb::context::Context;
use libbasicweb::types::config::Config;

use thiserror::Error;

use libbasicweb::webapp::{self, WebApp};

#[derive(Error, Debug)]
pub enum Error {
    #[error("ContextError")]
    ContextError(#[from] libbasicweb::context::Error),
    #[error("WebAppError")]
    WebAppError(#[from] webapp::AppError),
}

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    env_logger::init();

    let config = Config::default();

    let context = Context::new(&config).await?;

    let app = WebApp::new(context);
    app.run().await?;

    Ok(())
}
