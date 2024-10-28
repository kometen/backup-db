mod tests;

use crate::compression::Compression;
use anyhow::{Context, Result};
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
    /// Creates a new FileSystem instance with a specific value.
    ///
    /// # Arguments
    ///
    /// * `compression` - Name of the variable with information about compression value and type
    ///
    /// # Returns
    ///
    /// A Result containing the FileSystem if successful, or an error if the secret
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use backup_db::Compression;
    /// use backup_db::FileSystem;
    /// use anyhow::Result;
    ///
    /// fn example() -> Result<()> {
    ///     let filesystem = FileSystem::new(&Compression::new()?)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(compression: &Compression) -> Result<Self> {
        dotenv().ok();

        let file_prefix =
            env::var("FILE_PREFIX").context("Invalid FILE_PREFIX environment variable")?;

        let folder = env::var("FOLDER").context("Invalid FOLDER environment variable")?;

        let home = home_dir().context("Failed to determine home directory")?;

        let path = check_folder(&home, &folder.as_str())?;
        let filename = get_filename(&file_prefix, &path, &compression.compression_method)?;

        Ok(Self { filename })
    }
}

fn get_filename(
    file_prefix: &String,
    path: &PathBuf,
    compression_method: &String,
) -> Result<PathBuf> {
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

fn check_folder(home: &PathBuf, folder: &str) -> Result<PathBuf> {
    let path = Path::new(home).join(folder);
    let metadata = fs::metadata(&path)
        .with_context(|| format!("Error accessing folder: {}", path.display()))?;

    if metadata.is_dir() {
        Ok(path)
    } else {
        anyhow::bail!("Path exists but is not a directory: {}", path.display());
    }
}
