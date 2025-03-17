use std::env;
use std::net::SocketAddr;
use datasource::init_redis;
use settings::SETTINGS;
use tracing::info;
use tokio::net::TcpListener;

mod repositories;
mod controllers;
mod settings;
mod logger;
mod routes;
mod models;
mod utils;
mod dtos;
mod error;
mod prelude;
mod services;
mod datasource;
mod app;

use crate::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()>{
    dotenv().ok();
    logger::setup();
    init_redis().await;
    let port = SETTINGS.server.port;
    let address = SocketAddr::from(([127, 0, 0, 1], port));
    let listener = TcpListener::bind(address).await.unwrap();
    info!("Server listening on {}", address);
    let app = app::create_app().await;
    axum::serve(listener,app).await.unwrap();

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