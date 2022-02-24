use std::env;

#[derive(Clone)]
pub struct Config {
    pub address: String,
}

impl Config {
    pub fn new() -> Config {
        let _host = env::var("HOST")
            .unwrap_or("0.0.0.0".to_string());
        let _port = env::var("PORT")
            .unwrap_or("7777".to_string());

        Config {
            address: format!("{}:{}", _host, _port)
        }
    }
}
