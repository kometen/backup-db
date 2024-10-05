pub mod backup {

    use crate::compression::Compression;
    use crate::environment::Environment;
    use crate::filesystem::FileSystem;
    use crate::vault::Vault;
    use std::path::PathBuf;
    use std::process::Stdio;
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
    use tokio::process::Command;

    pub async fn perform_backup(
        compression: &Compression,
        env: &Environment,
        fs: &FileSystem,
        vault: &Vault,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut command = Command::new("pg_dump")
            .arg(&vault.connect_string)
            .arg(&compression.compression_parameter)
            .arg(&compression.compression_method)
            .arg(&compression.compression_parameter)
            .arg(&compression.compression_level)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = command.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout);
        let mut file = File::create(&fs.filename).await?;
        let mut buffer = vec![0; env.buffer_size];

        loop {
            let bytes_read = reader.read(&mut buffer).await?;

            if bytes_read == 0 {
                break;
            }

            file.write_all(&buffer[..bytes_read]).await?;
        }

        let timeout_duration = std::time::Duration::from_secs(60);
        let result = tokio::time::timeout(timeout_duration, command.wait()).await;
        match result {
            Ok(Ok(_status)) => { /* everyting is ok */ }
            Ok(Err(e)) => eprintln!("pg_dump failed with exit status: {:?}", e),
            Err(_) => eprintln!("pg_dump timed out"),
        }

        let file_path = PathBuf::from(&fs.filename);
        println!(
            "Backup successfully written to {} using {} compression at level {}.",
            &file_path.display(),
            &compression.compression_method,
            &compression.compression_level
        );

        Ok(())
    }
}
