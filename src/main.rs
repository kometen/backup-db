use azure_security_keyvault::prelude::*;
use dirs::home_dir;
use dotenv::dotenv;
use std::env;
use std::io::{BufReader, Read};
use std::process::{Command, Stdio};
use time::OffsetDateTime;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

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

    let db_host = client.get("db-host").await?.value;
    let db_user = client.get("db-user").await?.value;
    let db_name = client.get("db-name").await?.value;
    let db_pwd = client.get("db-pwd").await?.value;

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
        let bytes_read = reader.read(&mut buffer)?;

        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read]).await?;
    }

    println!("Backup successfully written to {}", filename);

    Ok(())
}
