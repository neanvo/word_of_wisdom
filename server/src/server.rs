use async_trait::async_trait;
use anyhow::Error;
use anyhow::Result;
use tokio::net::{TcpListener, TcpStream};
use common_lib::{Channel, AllowConnectionMessage, ChallengeSolutionMessage, Challenge, ChallengeRequestMessage, BookQuoteMessage};
use crate::config::{Config};
use crate::quotes::{Quotes};

pub struct Server {
    config: Config,
}

impl Server {
    pub fn new() -> Server { Server { config: Config::new() } }

    pub async fn start(self) -> Result<()> {
        let listener = TcpListener::bind(self.config.address.clone()).await?;
        log::info!("Starting server on {:?}", self.config.address.clone());

        while let Ok((mut tcp_stream, socket_addr)) = listener.accept().await {
            log::info!("{:?} connected", socket_addr);
            let c = self.config.clone();
            tokio::spawn(async move {
                let connection = TcpConnection{ config: c };
                if let Err(err) = connection.handle(&mut tcp_stream).await {
                    log::error!("Connection error: {:?}", err);
                }
            });
        }

        Ok(())
    }
}

struct TcpConnection {
    config: Config
}

#[async_trait]
trait TcpConnectionHandler {
    async fn handle(&self, stream: &mut TcpStream) -> Result<(), Error>;
}

#[async_trait]
impl TcpConnectionHandler for TcpConnection {
    async fn handle(&self, stream: &mut TcpStream) -> Result<(), Error> {
        let mut ch = Channel::new(stream);
        let challenge = Challenge::new(self.config.pow.difficulty);

        ch.send(&ChallengeRequestMessage {
            difficulty: challenge.difficulty,
            hash_seq: challenge.hash_seq,
        }).await?;

        let received = ch.receive::<ChallengeSolutionMessage>().await?;
        match challenge.verify(&received.solution) {
            Ok(_) => {
                ch.send(&AllowConnectionMessage{ is_allowed: true }).await?;
                log::info!("Challenge successfully completed!");
            }
            Err(err) => {
                log::error!("Solution rejected: {:?} \nWith err: {:?}", challenge, err);
                ch.shutdown().await?;
            }
        }

        let quotes = Quotes::from_file(self.config.quotes_path.clone())?;
        let rq = quotes.get_random_quote()?;
        ch.send(&BookQuoteMessage{ quote: rq.clone() }).await?;
        log::info!("Quote {:?} is sent", rq.clone());
        log::info!("Disconnected");

        Ok(())
    }
}
