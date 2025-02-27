use once_cell::sync::Lazy;
use config::ConfigError;

pub struct Logger {
    pub level: String,
}
pub struct Server {
    pub port: u16,
}

pub struct Settings {
    pub logger: Logger,
    pub server: Server
}

pub static SETTINGS: Lazy<Settings> = Lazy::new(|| Settings::new().expect("Failed to setup settings"));

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        Ok(Settings{ logger: Logger{ level: "DEBUG".to_owned() }, server: Server { port: 8080 } })
    }
}