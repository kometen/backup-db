use crate::environment::Environment;

pub struct FileSystem {
    pub filename: String,
}

impl FileSystem {
    pub fn new(env: &Environment) -> Result<Self, std::io::Error> {
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

        let compresion_suffix: String = match env.compression_method.as_str() {
            "none" => String::new(),
            _ => format!(".{}", env.compression_method),
        };

        let filename = format!(
            "{}/{}/{}-{}.dmp{}",
            home,
            folder,
            file_prefix,
            now.date(),
            compresion_suffix
        );

        Ok(Self { filename })
    }
}
