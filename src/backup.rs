pub mod backup {

    use crate::Compression;
    use crate::DatabaseConfig;
    use crate::Environment;
    use crate::FileSystem;
    use anyhow::Result;
    use azure_vault_secrets::Vault;
    use std::path::Path;
    use std::path::PathBuf;
    use std::process::Stdio;
    use tempfile::NamedTempFile;
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::process::Command;

    /// Performs the actual backup.
    ///
    /// # Example
    ///
    /// ```
    /// use backup_db::perform_backup;
    /// use db_config::db_config_from_vault;
    /// use backup_db::Compression;
    /// use backup_db::Environment;
    /// use backup_db::FileSystem;
    /// use azure_vault_secrets::{Vault, VaultStorage};
    ///
    /// async fn example() -> Result<(), Box<dyn std::error::Error>> {
    ///     db_config_from_vault!([host, user, name, pwd, domain]);
    ///
    ///     let db_keys = DatabaseConfig::db_keys();
    ///     let compression = Compression::new()?;
    ///     let environment = Environment::new()?;
    ///     let filesystem = FileSystem::new(&compression)?;
    ///     let vault = Vault::new("URL", db_keys).await?;
    ///     let _ = perform_backup(&compression, &environment, &filesystem, &vault);
    ///     Ok(())
    /// }
    /// ```
    pub async fn perform_backup(
        compression: &Compression,
        env: &Environment,
        fs: &FileSystem,
        vault: &Vault,
    ) -> Result<()> {
        // Create temporary file in the same directory as the target file
        let filename = &fs.filename;
        let dir = Path::new(&filename).parent().unwrap_or(Path::new("."));
        let temp_file = NamedTempFile::new_in(dir)?;
        let temp_path = temp_file.path().to_path_buf();
        let config = DatabaseConfig::from_vault(vault)?;

        let mut command = Command::new("pg_dump")
            .arg(&config.connection_string())
            .env("PGPASSWORD", &config.password())
            .arg(&compression.compression_parameter)
            .arg(format!(
                "{}:{}",
                &compression.compression_method, &compression.compression_level
            ))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let stdout = command.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout);
        let mut file = File::create(&temp_path).await?;
        let mut buffer = vec![0; env.buffer_size];

        loop {
            let bytes_read = reader.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read]).await?;
        }

        let status = command.wait().await?;

        if !status.success() {
            // Read stderr to get error message
            let mut stderr = String::new();
            if let Some(mut stderr_handle) = command.stderr {
                stderr_handle.read_to_string(&mut stderr).await?;
            }

            return Err(anyhow::anyhow!(
                "pg_dump failed with exit code: {:?}, error: {}",
                status.code(),
                stderr
            ));
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(&temp_path, std::fs::Permissions::from_mode(0o644))?;
            }

            // Ensure the target directory exists
            if let Some(parent) = Path::new(&filename).parent() {
                std::fs::create_dir_all(parent)?;
            }

            std::fs::rename(&temp_path, &filename)?;

            let file_path = PathBuf::from(&filename);
            println!(
                "Backup successfully written to {} using {} compression at level {}.",
                &file_path.display(),
                &compression.compression_method,
                &compression.compression_level
            );
        }

        Ok(())
    }
}
