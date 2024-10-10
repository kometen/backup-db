mod tests;
use std::env;
use std::{env::VarError, num::ParseIntError};

use dotenv::dotenv;

pub struct Environment {
    pub domain: String,
    pub buffer_size: usize,
}

impl Environment {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let domain = env::var("DOMAIN").expect("Missing DOMAIN environment variable.");
        let buffer_size = get_buffer_size(env::var("BUFFER_SIZE"))
            .map_err(|e| format!("Invalid BUFFER_SIZE: {}", e))?;

        Ok(Self {
            domain,
            buffer_size,
        })
    }
}

fn get_buffer_size(env: Result<String, VarError>) -> Result<usize, ParseIntError> {
    dotenv().ok();

    env.unwrap_or_else(|_| String::from("8192"))
        .parse::<usize>()
}
