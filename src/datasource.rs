use once_cell::sync::Lazy;
use redis::{aio::Connection, AsyncCommands};
use tokio::sync::RwLock;
use futures::{future::{join_all, try_join_all}, StreamExt};
use tracing::debug;
use crate::{error::Error, prelude::Result, settings::SETTINGS};

static REDIS_CONNECTION: Lazy<RwLock<Option<Connection>>> = Lazy::new(|| RwLock::new(None));

pub async fn init_redis() {
    let settings = SETTINGS.clone();
    let client = redis::Client::open(settings.redis.url)
        .expect("Cannot connect to redis");
    let con = client
        .get_async_connection()
        .await
        .expect("Cannot connect to redis");
    let mut connection = REDIS_CONNECTION.write().await;
    *connection = Some(con);
    debug!("Redis initialized");
}

pub async fn set_value(key: &str, value: &str) -> Result<()> {
    let mut connection = REDIS_CONNECTION.write().await;
    if let Some(connection) = &mut *connection {
        connection.set(key, value).await.map_err(|_| Error::FetchData)?;
        Ok(())
    } else {
        panic!("Redis is not initialized");
    }
}

async fn get_all_keys(pattern: &str) -> Result<Vec<String>> {
    let mut connection = REDIS_CONNECTION.write().await;
    if let Some(connection) = &mut *connection {
        let iter: redis::AsyncIter<String> = connection.scan_match(pattern).await.map_err(|_| Error::FetchData)?;
        let all_keys: Vec<String> = iter.collect().await;
        debug!("All keys: {:?}", all_keys);
        Ok(all_keys)
    } else {
        Err(Error::FetchData)
    }
}

pub async fn get_all(pattern: &str) -> Result<Vec<String>> {
    let all_keys = get_all_keys(pattern).await?;
    let results = try_join_all(
        all_keys.iter().map(|key| async move {
            let mut connection = REDIS_CONNECTION.write().await;
            if let Some(connection) = &mut *connection {
                let value: redis::RedisResult<String> = connection.get(key).await;
                Ok(value.map_err(|_| Error::FetchData)?)
            } else {
                Err(Error::FetchData)
            }
        })
    ).await?;
    debug!("All values: {:?}", results);
    Ok(results)
}