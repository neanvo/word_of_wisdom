use std::str;
use anyhow::Error;
use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

mod challenge;
pub use challenge::*;

pub type ChallengeSequence = [u8; 32];
pub type ChallengeSolution = [u8; 32];

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct ChallengeRequestMessage {
    pub difficulty: u8,
    pub hash_seq: ChallengeSequence,
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct ChallengeSolutionMessage {
    pub solution: ChallengeSolution
}

#[derive(Clone, Copy, Deserialize, Serialize, Debug)]
pub struct AllowConnectionMessage {
    pub is_allowed: bool
}

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct BookQuoteMessage {
    pub quote: String
}

pub struct Channel<'a> {
    pub s: &'a mut TcpStream,
}

impl<'a> Channel<'a> {
    pub fn new(s: &'a mut TcpStream) -> Self {
        Self { s }
    }

    pub async fn send<S>(&mut self, data: &S) -> Result<usize, Error> where S: serde::ser::Serialize {
        Ok(self.s.write(serde_json::to_string(data).unwrap().as_bytes()).await?)
    }

    pub async fn receive<R>(&mut self) -> Result<R, Error> where R: DeserializeOwned {
        let mut buf = vec![0; 1024];
        let read = self.s.read(&mut buf).await?;
        Ok(serde_json::from_str(str::from_utf8(&buf[0..read])?)?)
    }

    pub async fn shutdown(&mut self) -> Result<(), Error> {
        log::info!("Disconnecting...");
        Ok(self.s.shutdown().await?)
    }
}
