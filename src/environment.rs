pub struct Environment {
    pub buffer_size: usize,
    pub compression_method: String,
    pub compression_parameter: String,
    pub filename: String,
}

impl Environment {
    pub fn new() -> Result<Self, std::io::Error> {
        use dirs::home_dir;
        use dotenv::dotenv;
        use std::env;
        use time::OffsetDateTime;

        dotenv().ok();

        let file_prefix =
            env::var("FILE_PREFIX").expect("Missing FILE_PREFIX environment variable.");
        let folder = env::var("FOLDER").expect("Missing FOLDER environment variable.");
        let now = OffsetDateTime::now_utc();

        let home = home_dir()
            .unwrap_or_else(|| "".parse().unwrap())
            .into_os_string()
            .into_string()
            .unwrap();

        let compression_method =
            env::var("COMPRESSION_METHOD").unwrap_or_else(|_| "none".to_string());
        let compresion_suffix: String = match compression_method.as_str() {
            "none" => String::new(),
            _ => format!(".{}", compression_method),
        };

        let filename = format!(
            "{}/{}/{}-{}.dmp{}",
            home,
            folder,
            file_prefix,
            now.date(),
            compresion_suffix
        );

        let buffer_size = env::var("BUFFER_SIZE")
            .unwrap_or_else(|_| "8192".to_string())
            .parse::<usize>()
            .unwrap();

        Ok(Self {
            buffer_size,
            compression_method,
            compression_parameter: "-Z".to_string(),
            filename,
        })
    }
}
