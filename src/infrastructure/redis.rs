use tokio::sync::OnceCell;
use redis::{aio::MultiplexedConnection, Client, Commands, Connection};

static CLIENT: OnceCell<MultiplexedConnection> = OnceCell::const_new();

pub async fn connection() -> &'static MultiplexedConnection {
    CLIENT.get_or_init(|| async {
        let redis_client = redis::Client::open("redis://127.0.0.1/").expect("Error1");
        //let redis_connection_manager = redis_client.get_tokio_connection_manager().await.expect("Error2");
        let connection= redis_client.get_multiplexed_async_connection().await.expect("Error getting tokio connection");
        connection
        //client.get_multiplexed_async_connection().await.expect("cannot get connection")
    }).await
}
