pub struct Environment {
    pub buffer_size: usize,
}

impl Environment {
    pub fn new() -> Result<Self, std::io::Error> {
        use dotenv::dotenv;
        use std::env;

        dotenv().ok();

        let buffer_size = env::var("BUFFER_SIZE")
            .unwrap_or_else(|_| "8192".to_string())
            .parse::<usize>()
            .unwrap();

        Ok(Self { buffer_size })
    }
}
