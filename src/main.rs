use backup_db::{check_dns, perform_backup, Compression, Environment, FileSystem, Vault};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let compression = Compression::new()?;
    let env = Environment::new()?;
    let fs = FileSystem::new(&compression)?;
    let vault = Vault::new().await?;
    check_dns(&vault, &env).await?;
    perform_backup(&compression, &env, &fs, &vault).await?;

    Ok(())
}
