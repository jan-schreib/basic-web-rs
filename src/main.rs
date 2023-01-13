pub mod context;
pub mod db;
pub mod gtypes;

use context::Context;
use gtypes::config::GConfig;

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let config = GConfig {
        db_url: "test.db".to_string(),
        cache_url: String::new(),
    };

    let _context = Context::new(&config).await.unwrap();
    Ok(())
}
