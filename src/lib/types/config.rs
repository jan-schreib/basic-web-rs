use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub cache_url: String,
    pub addr: SocketAddr,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: "sqlite::memory:".to_string(),
            cache_url: Default::default(),
            addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
        }
    }
}
