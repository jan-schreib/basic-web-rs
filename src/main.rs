use std::net::SocketAddr;

use libbasicweb::types::config::Config;
use libbasicweb::{context::Context, types::config::Oauth2};

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

    let config = Config {
        db_url: "sqlite::memory:".to_string(),
        cache_url: String::new(),
        addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
        oauth2: Oauth2::default(),
    };

    let context = Context::new(&config).await?;

    let app = WebApp::new(context);
    app.run().await?;

    Ok(())
}
