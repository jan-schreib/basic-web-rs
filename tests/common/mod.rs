use libgout::{
    context::Context,
    types::{config::Config, food::Food},
};
use rand::Rng;
use uuid::Uuid;

pub async fn create_test_context() -> Context {
    let config = Config::default();

    let context = Context::new(&config)
        .await
        .expect("Failed to generate testcontext");

    context
        .run_migrations()
        .await
        .expect("Failed to run test migrations");

    context
}

pub fn generate_random_food() -> Food {
    let mut rng = rand::thread_rng();
    Food {
        id: rng.gen(),
        name: Uuid::new_v4().to_string(),
        kcal: 1,
        purine: 4,
        uric_acid: None,
        gout_factor: None,
    }
}

pub fn generate_random_foods(n: u64) -> Vec<Food> {
    let mut foods = Vec::new();
    for _ in 0..n {
        foods.push(generate_random_food())
    }
    foods
}
