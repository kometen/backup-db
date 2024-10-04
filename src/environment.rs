pub struct Environment {
    pub buffer_size: usize,
    pub compression_method: String,
    pub compression_parameter: String,
}

impl Environment {
    pub fn new() -> Result<Self, std::io::Error> {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();

        let compression_method =
            env::var("COMPRESSION_METHOD").unwrap_or_else(|_| "none".to_string());

        let buffer_size = env::var("BUFFER_SIZE")
            .unwrap_or_else(|_| "8192".to_string())
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            buffer_size,
            compression_method,
            compression_parameter: "-Z".to_string(),
        })
    }
}
