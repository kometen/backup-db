mod tests;
use std::env;
use std::{env::VarError, num::ParseIntError};

use dotenv::dotenv;

pub struct Environment {
    pub buffer_size: usize,
}

impl Environment {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let buffer_size = get_buffer_size(env::var("BUFFER_SIZE"))
            .map_err(|e| format!("Invalid BUFFER_SIZE: {}", e))?;

        Ok(Self { buffer_size })
    }
}

fn get_buffer_size(env: Result<String, VarError>) -> Result<usize, ParseIntError> {
    dotenv().ok();

    env.unwrap_or_else(|_| String::from("8192"))
        .parse::<usize>()
}
