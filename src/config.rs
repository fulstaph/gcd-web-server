extern crate dotenv;

use dotenv::dotenv;
use std::str::FromStr;

pub struct Config {
    port: u64
}

impl Config {
    pub fn get_port(self: &Self) -> String {
        format!(":{}", self.port)
    }
}

pub fn get_config() -> Config {
    dotenv().ok();

    Config {
        // TODO: proper error handling Kappa
        port: u64::from_str(dotenv::var("PORT").unwrap().as_str()).unwrap()
    }
}