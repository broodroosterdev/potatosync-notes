pub use ::config::ConfigError;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server_addr: String,
    pub database_url: String,
    pub jwt_secret: String
}
impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}