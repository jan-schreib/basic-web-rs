pub mod test_context;

use anyhow::Result;
use std::time::Duration;

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
        id: rng.gen_range(1..200),
        name: Uuid::new_v4().to_string(),
        kcal: 1,
        purine: 4,
        uric_acid: Some(0.0),
        gout_factor: Some(2),
    }
}

pub fn generate_random_foods(n: u64) -> Vec<Food> {
    let mut foods = Vec::new();
    for _ in 0..n {
        foods.push(generate_random_food())
    }
    foods
}

fn create_http_client() -> reqwest::Client {
    reqwest::Client::builder()
        .user_agent("goutdb-test-client")
        .timeout(Duration::from_millis(1_000))
        .build()
        .unwrap()
}

pub async fn get(url: String) -> Result<reqwest::Response> {
    let client = create_http_client();

    Ok(client.get(url).send().await?)
}
