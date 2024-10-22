use anyhow::Result;
use backup_db::{
    check_dns, perform_backup, Compression, Environment, FileSystem, SecretManager, Vault,
};

#[tokio::main]
async fn main() -> Result<()> {
    let compression = Compression::new()?;
    let env = Environment::new()?;
    let fs = FileSystem::new(&compression)?;
    let secret_manager = SecretManager::new()?;
    let vault = Vault::new(secret_manager.url).await?;
    if let Err(e) = check_dns(&vault, &env).await {
        //eprintln!("DNS resolution failed: {}", e);
        //eprintln!("Root cause: {}", e.root_cause());
        return Err(e);
    }

    let _ = perform_backup(&compression, &env, &fs, &vault).await;

    Ok(())
}
