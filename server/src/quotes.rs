use std::fs::{File};
use std::io::{BufReader};
use serde::{Serialize, Deserialize};
use anyhow::Error;
use rand::Rng;

pub type QuotesArray = Vec<String>;

#[derive(Serialize, Deserialize)]
pub struct Quotes {
    pub arr: QuotesArray,
}

impl Quotes {
    pub fn from_file(path: String) -> Result<Quotes, Error> {
        log::info!("QUOTES_PATH: {:?}", path);
        let file = File::open(path.as_str()).unwrap();
        let reader = BufReader::new(file);
        let arr = serde_json::from_reader::<BufReader<_>, QuotesArray>(reader)?;
        Ok(Quotes{ arr })
    }

    pub fn get_random_quote(&self) -> Result<String, Error> {
        let rand_quote_index = rand::thread_rng().gen_range(0..(self.arr.len() - 1));
        Ok(self.arr[rand_quote_index].clone())
    }
}
