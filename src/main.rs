pub mod context;
pub mod db;
pub mod gtypes;

use db::interface::Db;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("DbError")]
    DbError(#[from] db::error::DbError),
    #[error("ContextError")]
    ContextError(#[from] context::Error),
}

use context::Context;
use gtypes::config::GConfig;

use crate::gtypes::food::FoodInsert;

#[tokio::main]
pub async fn main() -> Result<(), Error> {
    let config = GConfig {
        db_url: "sqlite::memory:".to_string(),
        cache_url: String::new(),
    };

    let context = Context::new(&config).await?;

    context.run_migrations().await?;

    let db = context.database();

    let foods = db.get_foods().await?;

    foods.iter().for_each(|f| println!("{}", f));
    db.add_food(FoodInsert {
        name: "Kiwi".to_string(),
    })
    .await?;
    db.delete_food(4).await?;
    Ok(())
}
