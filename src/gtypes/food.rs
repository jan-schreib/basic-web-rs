use derive_more::Display;

#[derive(Default, Display, sqlx::FromRow)]
#[display(fmt = "ID: {}\nName: {}", id, name)]
pub struct Food {
    pub id: i64,
    pub name: String,
}

#[derive(Default)]
pub struct FoodInsert {
    pub name: String,
}
