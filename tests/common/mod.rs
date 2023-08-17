pub mod test_context;

use anyhow::Result;
use serde::Serialize;
use std::time::Duration;

use libbasicweb::{context::Context, types::config::Config};
use rand::Rng;

pub async fn create_test_context() -> Context {
    let mut rng = rand::thread_rng();
    let mut config = Config::default();
    config.addr.set_port(rng.gen_range(3001..4001));

    let context = Context::new(&config)
        .await
        .expect("Failed to generate testcontext");

    context
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

pub async fn post<T: Serialize>(url: String, payload: T) -> Result<reqwest::Response> {
    let client = create_http_client();

    Ok(client.post(url).json(&payload).send().await?)
}

pub async fn delete(url: String) -> Result<reqwest::Response> {
    let client = create_http_client();

    Ok(client.delete(url).send().await?)
}
