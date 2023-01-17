use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Display, sqlx::FromRow, Clone, Serialize)]
#[display(fmt = "ID: {}\nName: {}", id, name)]
pub struct Food {
    pub id: i64,
    pub name: String,
}

#[derive(Default, Deserialize)]
pub struct FoodInsert {
    pub name: String,
}

impl From<Food> for FoodInsert {
    fn from(value: Food) -> Self {
        Self { name: value.name }
    }
}
