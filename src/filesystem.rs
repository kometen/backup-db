use crate::compression::Compression;
use dirs::home_dir;
use dotenv::dotenv;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub struct FileSystem {
    pub filename: PathBuf,
}

impl FileSystem {
    pub fn new(compression: &Compression) -> Result<Self, std::io::Error> {
        dotenv().ok();

        let file_prefix =
            env::var("FILE_PREFIX").expect("Missing FILE_PREFIX environment variable.");
        let folder = env::var("FOLDER").expect("Missing FOLDER environment variable.");
        let now = OffsetDateTime::now_utc();

        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());

        let compresion_suffix: String = match compression.compression_method.as_str() {
            "none" => String::new(),
            _ => format!(".{}", compression.compression_method),
        };

        let path = check_folder(&home, &folder.as_str())?;

        let filename = Path::new(&path).join(format!(
            "{}-{}.dmp{}",
            file_prefix,
            now.date(),
            compresion_suffix
        ));

        Ok(Self { filename })
    }
}

fn check_folder(home: &PathBuf, folder: &str) -> Result<PathBuf, std::io::Error> {
    let path = Path::new(home).join(folder);
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(path)
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Path exists but is not a directory: {}", path.display()),
                ))
            }
        }
        Err(e) => Err(std::io::Error::new(
            e.kind(),
            format!("Error accessing folder {}: {}", path.display(), e),
        )),
    }
}
