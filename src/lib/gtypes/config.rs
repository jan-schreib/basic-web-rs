use std::net::SocketAddr;

#[derive(Clone)]
pub struct GConfig {
    pub db_url: String,
    pub cache_url: String,
    pub port: SocketAddr,
}

impl Default for GConfig {
    fn default() -> Self {
        Self {
            db_url: "sqlite::memory:".to_string(),
            cache_url: Default::default(),
            port: SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    }
}
