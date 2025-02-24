use std::env;
use tracing::info;

mod settings;
mod logger;

fn main() {
    logger::setup();
    info!("Hello, world!");
    info!("{}", env::var("RUST_LOG").unwrap());
}
