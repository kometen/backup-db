use std::env::VarError;

pub struct Compression {
    pub compression_level: String,
    pub compression_method: String,
    pub compression_parameter: String,
}

impl Compression {
    pub fn new() -> Result<Self, std::io::Error> {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();

        let compression_method = get_compression_method(env::var("COMPRESSION_METHOD"));

        let compression_level = get_compression_level(env::var("COMPRESSION_LEVEL"));

        Ok(Self {
            compression_level,
            compression_method,
            compression_parameter: "-Z".to_string(),
        })
    }
}

// Return-type is compression method and compression level.
fn get_compression_method(env: Result<String, VarError>) -> String {
    let r = match env.unwrap_or_else(|_| "none".to_string()).as_str() {
        "gzip" => "gzip",
        "lz4" => "lz4",
        "zstd" => "zstd",
        "none" => "none",
        _ => "none",
    };

    r.to_string()
}

fn get_compression_level(env: Result<String, VarError>) -> String {
    use range_check::Check;

    env.unwrap_or_else(|_| "0".to_string())
        .parse::<u8>()
        .unwrap()
        .check_range(0..10)
        .unwrap_or(0)
        .to_string()
}
