use crate::create_db_structs;

create_db_structs! {Food, FoodInsert, name: String, kcal: i64, purine: i64, uric_acid: Option<f64>, gout_factor: Option<i64>}

impl From<Food> for FoodInsert {
    fn from(value: Food) -> Self {
        Self {
            name: value.name,
            kcal: value.kcal,
            purine: value.purine,
            uric_acid: value.uric_acid,
            gout_factor: value.gout_factor,
        }
    }
}

pub struct A {
    pub name: String,
    pub kcal: i64,
    pub puring: i64,
    pub uric_acid: Option<f64>,
    pub gout_factor: Option<i64>
}