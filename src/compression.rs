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

        let compression_level =
            get_compression_level(env::var("COMPRESSION_LEVEL"), compression_method.1);

        Ok(Self {
            compression_level,
            compression_method: compression_method.0,
            compression_parameter: "-Z".to_string(),
        })
    }
}

// Return-type is compression method and default compression level.
fn get_compression_method(env: Result<String, VarError>) -> (String, u8) {
    let r = match env.unwrap_or_else(|_| "none".to_string()).as_str() {
        "gzip" => ("gzip".to_string(), 6),
        "lz4" => ("lz4".to_string(), 1),
        "zstd" => ("zstd".to_string(), 3),
        "none" => ("none".to_string(), 0),
        _ => ("none".to_string(), 0),
    };

    r
}

fn get_compression_level(env: Result<String, VarError>, default_compression_level: u8) -> String {
    use range_check::Check;

    env.unwrap_or_else(|_| default_compression_level.to_string())
        .parse::<u8>()
        .unwrap()
        .check_range(0..10)
        .unwrap_or(default_compression_level)
        .to_string()
}
