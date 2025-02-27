use std::env;
use std::net::SocketAddr;
use app::create_app;
use settings::SETTINGS;
use tracing::info;
use tokio::net::TcpListener;

mod settings;
mod logger;
mod routes;
mod models;
mod utils;
mod dtos;
mod app;

#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    logger::setup();
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address).await?;
    info!("Server listening on {}", address);
    let app = app::create_app().await;
    axum::serve(listener,app).await?;

    info!("Hello, world!");
    info!("{}", env::var("RUST_LOG").unwrap());
    Ok(())
}

/* fn main() -> Result<(), std::io::Error> {
    let rt = tokio::runtime::Runtime::new().unwrap();
    logger::setup();
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let future = TcpListener::bind(address);
    let listener = rt.block_on(future)?;

    info!("Server listening on {}", address);

    info!("Hello, world!");
    info!("{}", env::var("RUST_LOG").unwrap());
    Ok(())
}
 */