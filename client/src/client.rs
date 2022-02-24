use anyhow::Error;
use tokio::net::TcpStream;
use common_lib::{AllowConnectionMessage, BookQuoteMessage, Challenge, ChallengeRequestMessage, ChallengeSolutionMessage, Channel};
use crate::config::Config;

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new() -> Client { Client { config: Config::new() } }

    pub async fn start(self) -> Result<(), Error> {
        let mut stream = TcpStream::connect(self.config.address.clone()).await?;
        let mut ch = Channel::new(&mut stream);
        log::info!("Client started. Connecting to {:?}", self.config.address.clone());

        let crm = ch.receive::<ChallengeRequestMessage>().await?;
        let mut challenge = Challenge::of(crm);

        challenge.solve()?;
        ch.send(&ChallengeSolutionMessage{ solution: challenge.solution.unwrap() }).await?;

        let acm = ch.receive::<AllowConnectionMessage>().await?;
        if acm.is_allowed {
            let bqm = ch.receive::<BookQuoteMessage>().await?;
            log::info!("{:?}", bqm.quote)
        }

        ch.shutdown().await?;
        Ok(())
    }
}
