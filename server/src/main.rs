use anyhow::{Result};
use crate::logging::Logger;
use crate::server::{Server};

mod server;
mod config;
mod quotes;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init();
    let server = Server::new();
    return match server.start().await {
        Ok(()) => {
            Ok(())
        }
        Err(err) => {
            log::info!("Unexpected error happened: {:?}", err);
            Err(err)
        }
    };
}
