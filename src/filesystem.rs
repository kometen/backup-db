mod tests;

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

        let file_prefix = env::var("FILE_PREFIX")
            .map_err(|e| format!("Invalid FILE_PREFIX: {}", e))
            .unwrap();
        let folder = env::var("FOLDER")
            .map_err(|e| format!("Invalid FOLDER: {}", e))
            .unwrap();
        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());

        let path = check_folder(&home, &folder.as_str())?;
        let filename = get_filename(&file_prefix, &path, &compression.compression_method)?;

        Ok(Self { filename })
    }
}

fn get_filename(
    file_prefix: &String,
    path: &PathBuf,
    compression_method: &String,
) -> Result<PathBuf, std::io::Error> {
    let now = OffsetDateTime::now_utc();

    let compresion_suffix: String = match compression_method.as_str() {
        "none" => String::new(),
        _ => format!(".{}", compression_method),
    };

    Ok(Path::new(&path).join(format!(
        "{}-{}.dmp{}",
        file_prefix,
        now.date(),
        compresion_suffix
    )))
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
