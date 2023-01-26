use std::net::IpAddr;

use config::{Config, ConfigError, Environment, File};
use lazy_static::lazy_static;
use serde_derive::Deserialize;

lazy_static! {
    pub static ref SETTINGS: Settings = Settings::read().unwrap();
}

#[derive(Debug, Deserialize)]
pub struct SocketAddrConfig {
    pub host: IpAddr,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct MongoDBConfig {
    pub connection_string: String,
    pub database: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub grpc: SocketAddrConfig,
    pub mongodb: MongoDBConfig,
}

impl Settings {
    pub fn read() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name("./src/settings/config/default"))
            .add_source(File::with_name("./src/settings/config/local").required(false))
            .add_source(Environment::with_prefix("app"))
            .build()?
            .try_deserialize()
    }
}
