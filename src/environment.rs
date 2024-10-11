mod tests;
use anyhow::{Context, Result};
use std::env;

pub struct Environment {
    pub domain: String,
    pub buffer_size: usize,
}

impl Environment {
    pub fn new() -> Result<Self> {
        let domain = env::var("DOMAIN").context("Failed to get DOMAIN")?;
        let buffer_size =
            get_buffer_size(env::var("BUFFER_SIZE").context("Failed to get BUFFER_SIZE")?)
                .context("Failed to parse buffer size")?;

        Ok(Self {
            domain,
            buffer_size,
        })
    }
}

fn get_buffer_size(env: String) -> Result<usize> {
    let buffer_size = env
        .parse::<usize>()
        .with_context(|| format!("Invalid buffer size: {}", env))
        .unwrap_or(8192);

    Ok(buffer_size)
}
