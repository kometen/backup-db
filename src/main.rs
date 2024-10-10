use anyhow::Result;
use backup_db::{check_dns, perform_backup, Compression, Environment, FileSystem, Vault};

#[tokio::main]
async fn main() -> Result<()> {
    let compression = Compression::new().unwrap();
    let env = Environment::new().unwrap();
    let fs = FileSystem::new(&compression).unwrap();
    let vault = Vault::new().await?;
    if let Err(e) = check_dns(&vault, &env).await {
        //eprintln!("DNS resolution failed: {}", e);
        //eprintln!("Root cause: {}", e.root_cause());
        return Err(e);
    }

    let _ = perform_backup(&compression, &env, &fs, &vault).await;

    Ok(())
}
