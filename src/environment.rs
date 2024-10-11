mod tests;
use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub domain: String,
    pub buffer_size: usize,
}

impl Environment {
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let domain = env::var("DOMAIN").context("Failed to get DOMAIN")?;

        let buffer_size = env::var("BUFFER_SIZE")
            .map(|s| get_buffer_size(s))
            .unwrap_or(Ok(8192))?;

        Ok(Self {
            domain,
            buffer_size,
        })
    }
}

fn get_buffer_size(env: String) -> Result<usize> {
    let buffer_size = env
        .parse::<usize>()
        .context(format!("Invalid BUFFER_SIZE value: {}", env))?;

    Ok(buffer_size)
}
