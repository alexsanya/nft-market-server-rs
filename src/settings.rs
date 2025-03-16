use std::env;

use dotenv::dotenv;
use once_cell::sync::Lazy;
use config::ConfigError;

#[derive(Clone)]
pub struct Logger {
    pub level: String,
}
#[derive(Clone)]
pub struct Server {
    pub port: u16,
}

#[derive(Clone)]
pub struct Redis {
    pub url: String,
}

#[derive(Clone)]
pub struct ChainData {
    pub broker_address: String,
    pub private_key: String,
    pub rpc_url: String,
}

#[derive(Clone)]
pub struct Settings {
    pub logger: Logger,
    pub server: Server,
    pub redis: Redis,
    pub chain_data: ChainData
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().expect("Failed to setup settings"));

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        let chain_data = ChainData {
            broker_address: env::var("BROKER_ADDRESS").expect("BROKER_ADDRESS is not defined"),
            private_key: env::var("PRIVATE_KEY").expect("PRIVATE_KEY not set"),
            rpc_url: env::var("ETH_RPC_URL").expect("ETH_RPC_URL not set")
        };
        Ok(
            Settings {
                logger: Logger{ level: "DEBUG".to_owned() },
                server: Server { port: env::var("PORT").expect("PORT is not defined").parse().unwrap() },
                redis: Redis { url: env::var("REDIS_URL").expect("REDIS_URL is not set") },
                chain_data
            }
        )
    }
}