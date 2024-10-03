use azure_security_keyvault::prelude::*;
use dirs::home_dir;
use dotenv::dotenv;
use std::env;
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

    let output = Command::new("pg_dump")
        .arg(&connect_string)
        .stdout(Stdio::piped())
        .output()?;

    let f = File::create(&filename).await;

    let file_status = f
        .expect(format!("Unable to open file {}", &filename).as_str())
        .write_all(&output.stdout)
        .await;

    println!("output: {:?}", file_status);

    Ok(())
}
