extern crate env_logger;

use log::info;

pub struct Logger {}

impl Logger {
    pub fn init() -> () {
        env_logger::init();
        info!("starting with info only logs");
    }
}
