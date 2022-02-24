use anyhow::Result;
use crate::client::Client;
use crate::logging::Logger;

mod client;
mod config;
mod logging;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init();
    let client = Client::new();
    return match client.start().await {
        Ok(()) => {
            Ok(())
        }
        Err(err) => {
            log::info!("Unexpected error happened: {:?}", err);
            Err(err)
        }
    };
}
