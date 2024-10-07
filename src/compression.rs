mod tests;

use range_check::OutOfRangeError;
use std::{env::VarError, num::ParseIntError};

pub struct Compression {
    pub compression_level: String,
    pub compression_method: String,
    pub compression_parameter: String,
}

impl Compression {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();

        let compression_method_and_level =
            get_compression_method_and_level(env::var("COMPRESSION_METHOD"));

        let compression_level = get_compression_level(
            env::var("COMPRESSION_LEVEL").map_err(|e| format!("Invalid COMPRESSION_LEVEL: {}", e)),
            compression_method_and_level.1,
        )?;

        let compression_level = check_compression_level_is_in_range(compression_level)?;

        Ok(Self {
            compression_level: compression_level.to_string(),
            compression_method: compression_method_and_level.0,
            compression_parameter: "-Z".to_string(),
        })
    }
}

// Return-type is compression method and default compression level.
fn get_compression_method_and_level(env: Result<String, VarError>) -> (String, u8) {
    let r = match env.unwrap_or_else(|_| "none".to_string()).as_str() {
        "gzip" => ("gzip".to_string(), 6),
        "lz4" => ("lz4".to_string(), 1),
        "zstd" => ("zstd".to_string(), 3),
        "none" => ("none".to_string(), 0),
        _ => ("none".to_string(), 0),
    };

    r
}

fn get_compression_level(
    env: Result<String, String>,
    default_compression_level: u8,
) -> Result<u8, ParseIntError> {
    env.unwrap_or_else(|_| default_compression_level.to_string())
        .parse::<u8>()
}

fn check_compression_level_is_in_range(level: u8) -> Result<u8, OutOfRangeError<u8>> {
    use range_check::Check;

    level.check_range(0..10)
}
