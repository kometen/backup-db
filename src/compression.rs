mod tests;

use anyhow::{Context, Result};
use range_check::OutOfRangeError;
use std::env;

pub struct Compression {
    pub compression_level: String,
    pub compression_method: String,
    pub compression_parameter: String,
}

impl Compression {
    pub fn new() -> Result<Self> {
        use dotenv::dotenv;

        dotenv().ok();

        let compression_method_and_level = get_compression_method_and_level(
            env::var("COMPRESSION_METHOD").context("Failed to get COMPRESSION_METHOD")?,
        );

        let compression_level = get_compression_level(
            env::var("COMPRESSION_LEVEL").context("Failed to get COMPRESSION_LEVEL")?,
            compression_method_and_level.1,
        )
        .context("Failed to parse compression level")?;

        let compression_level = check_compression_level_is_in_range(compression_level)?;

        Ok(Self {
            compression_level: compression_level.to_string(),
            compression_method: compression_method_and_level.0,
            compression_parameter: "-Z".to_string(),
        })
    }
}

// Return-type is compression method and default compression level.
fn get_compression_method_and_level(env: String) -> (String, u8) {
    match env.as_str() {
        "gzip" => ("gzip".to_string(), 6),
        "lz4" => ("lz4".to_string(), 1),
        "zstd" => ("zstd".to_string(), 3),
        "none" => ("none".to_string(), 0),
        _ => ("none".to_string(), 0),
    }
}

fn get_compression_level(env: String, default_compression_level: u8) -> Result<u8> {
    let compression_level = env
        .parse::<u8>()
        .with_context(|| format!("Invalid compression level: {}", env))
        .unwrap_or(default_compression_level);

    Ok(compression_level)
}

fn check_compression_level_is_in_range(level: u8) -> Result<u8, OutOfRangeError<u8>> {
    use range_check::Check;

    level.check_range(0..10)
}
