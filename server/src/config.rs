use std::env;

#[derive(Clone)]
pub struct Config {
    pub address: String,
    pub pow: PowConfig,
    pub quotes_path: String
}

#[derive(Clone, Copy)]
pub struct PowConfig {
    pub difficulty: u8,
}

impl Config {
    pub fn new() -> Config {
        let _host = env::var("HOST")
            .unwrap_or("0.0.0.0".to_string());
        let _port = env::var("PORT")
            .unwrap_or("7777".to_string());
        let _pow_difficulty = env::var("POW_DIFFICULTY")
            .map_or(2, |s| s.parse::<u8>().unwrap());
        let _quotes_path = env::var("QUOTES_PATH")
            .unwrap_or("./server/static/quotes.json".to_string());

        Config {
            address: format!("{}:{}", _host, _port),
            pow: PowConfig{
                difficulty: _pow_difficulty,
            },
            quotes_path: _quotes_path
        }
    }
}
