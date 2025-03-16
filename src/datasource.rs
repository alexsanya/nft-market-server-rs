use once_cell::sync::Lazy;
use redis::Connection;
use tokio::sync::Mutex;
use crate::{error::Error, prelude::Result, settings::SETTINGS};

static REDIS_DATASOURCE: Lazy<Mutex<Connection>> = Lazy::new(|| Mutex::new(init_redis()));

fn init_redis() -> Connection {
    let settings = SETTINGS.clone();
    let connection = redis::Client::open(settings.redis.url)
        .expect("Cannot open connection to redis")
        .get_connection()
        .expect("cannot get connection");
    connection
}

pub async fn set_value(key: &str, value: &str) -> Result<()> {
    let mut connection = REDIS_DATASOURCE.try_lock().map_err(|_| Error::FetchData)?;
    let _: () = redis::cmd("SET").arg(key).arg(value).query(&mut connection).expect("Cannot write data");
    Ok(())
}