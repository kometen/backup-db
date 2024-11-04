use anyhow::Result;
use azure_vault_secrets::{Vault, VaultStorage};
use backup_db::{check_dns, perform_backup, Compression, Environment, FileSystem, SecretManager};
use clap::Parser;
use db_config::db_config_from_vault;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    namespace: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    db_config_from_vault!([host, user, name, pwd, domain]);

    let compression = Compression::new()?;
    let env = Environment::new()?;
    let fs = FileSystem::new(&compression)?;
    let secret_manager = SecretManager::new(cli.namespace.as_str())?;
    let db_keys = DatabaseConfig::db_keys();
    let vault = Vault::new(secret_manager.url.as_str(), db_keys).await?;
    if let Err(e) = check_dns(&vault).await {
        eprintln!("DNS resolution failed: {}", e);
        eprintln!("Root cause: {}", e.root_cause());
        return Err(e);
    }

    let _ = perform_backup(&compression, &env, &fs, &vault).await;

    Ok(())
}
