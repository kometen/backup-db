use backup_db::{
    backup::backup::perform_backup, compression::Compression, environment::Environment,
    filesystem::FileSystem, vault::Vault,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let compression = Compression::new()?;
    let env = Environment::new()?;
    let fs = FileSystem::new(&compression)?;
    let vault = Vault::new().await?;
    perform_backup(&compression, &env, &fs, &vault).await?;

    Ok(())
}
