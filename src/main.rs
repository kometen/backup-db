use azure_security_keyvault::prelude::*;
use dirs::home_dir;
use dotenv::dotenv;
use std::env;
use std::process::Stdio;
use time::OffsetDateTime;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let now = OffsetDateTime::now_utc();

    let domain = env::var("DOMAIN").expect("Missing DOMAIN environment variable.");
    let keyvault_url =
        env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
    let file_prefix = env::var("FILE_PREFIX").expect("Missing FILE_PREFIX environment variable.");

    let credential = azure_identity::create_credential()?;
    let client = SecretClient::new(&keyvault_url, credential)?;

    let db_host = client
        .get("db-host")
        .await
        .map_err(|e| format!("Error fetching db-host: {}", e))?
        .value;

    let db_user = client
        .get("db-user")
        .await
        .map_err(|e| format!("Error fetching db-user: {}", e))?
        .value;

    let db_name = client
        .get("db-name")
        .await
        .map_err(|e| format!("Error fetching db-name: {}", e))?
        .value;

    let db_pwd = client
        .get("db-pwd")
        .await
        .map_err(|e| format!("Error fetching db-pwd: {}", e))?
        .value;

    let connect_string = format!(
        "postgres://{}:{}@{}.{}/{}",
        db_user, db_pwd, db_host, domain, db_name
    );

    let home = home_dir()
        .unwrap_or_else(|| "".parse().unwrap())
        .into_os_string()
        .into_string()
        .unwrap();
    let folder = "data";
    let filename = format!("{}/{}/{}-{}.dmp", home, folder, file_prefix, now.date());

    let mut command = Command::new("pg_dump")
        .arg(&connect_string)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = command.stdout.take().expect("Failed to capture stdout");
    let mut reader = BufReader::new(stdout);

    let mut file = File::create(&filename).await?;
    let mut buffer = [0; 8192];

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

    println!("Backup successfully written to {}", filename);

    Ok(())
}
