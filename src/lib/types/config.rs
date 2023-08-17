use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub struct Config {
    pub db_url: String,
    pub cache_url: String,
    pub addr: SocketAddr,
    pub oauth2: Oauth2,
}

#[derive(Clone, Debug, Default)]
pub struct Oauth2 {
    pub redirect_url: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_url: "sqlite::memory:".to_string(),
            cache_url: Default::default(),
            addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            oauth2: Oauth2 {
                redirect_url: "http://localhost:3000".to_string(),
            },
        }
    }
}
