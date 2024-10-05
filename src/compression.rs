pub struct Compression {
    pub compression_level: String,
    pub compression_method: String,
    pub compression_parameter: String,
}

impl Compression {
    pub fn new() -> Result<Self, std::io::Error> {
        use dotenv::dotenv;
        use range_check::Check;
        use std::env;

        dotenv().ok();

        let compression_level = env::var("COMPRESSION_LEVEL")
            .unwrap_or_else(|_| "4".to_string())
            .parse::<u8>()
            .unwrap()
            .check_range(0..10)
            .unwrap_or(4)
            .to_string();

        let compression_method = match env::var("COMPRESSION_METHOD")
            .unwrap_or_else(|_| "none".to_string())
            .as_str()
        {
            "gzip" => "gzip",
            "lz4" => "lz4",
            "zstd" => "zstd",
            "none" => "none",
            _ => "none",
        };

        Ok(Self {
            compression_level,
            compression_method: compression_method.to_string(),
            compression_parameter: "-Z".to_string(),
        })
    }
}
