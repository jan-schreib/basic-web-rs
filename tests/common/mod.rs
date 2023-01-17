use libgout::{
    context::Context,
    gtypes::{config::GConfig, food::Food},
};
use rand::Rng;
use uuid::Uuid;

pub async fn create_test_context() -> Context {
    let config = GConfig::default();

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
    }
}

pub fn generate_random_foods(n: u64) -> Vec<Food> {
    let mut foods = Vec::new();
    for _ in 0..n {
        foods.push(generate_random_food())
    }
    foods
}
