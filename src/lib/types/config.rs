use std::net::SocketAddr;

#[derive(Clone, Debug)]
pub enum DbType {
    Sqlite,
    Postgres,
}

#[derive(Clone, Debug)]
pub struct DbConfig {
    pub db_type: DbType,
    pub db_url: String,
}

impl DbConfig {
    pub fn default_for(db_type: DbType) -> Self {
         match db_type {
            DbType::Sqlite => Self {
                db_type: DbType::Sqlite,
                db_url: "sqlite::memory:".to_string(),
            },
            DbType::Postgres => Self {
                db_type: DbType::Postgres,
                db_url: "postgres://postgres:postgres@localhost/postgres".to_string()
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Config {
    pub db: DbConfig,
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
            db: DbConfig::default_for(DbType::Sqlite),
            cache_url: Default::default(),
            addr: SocketAddr::from(([127, 0, 0, 1], 3000)),
            oauth2: Oauth2 {
                redirect_url: "http://localhost:3000".to_string(),
            },
        }
    }
}
