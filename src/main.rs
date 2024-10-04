use backup_db::{
    backup::backup::perform_backup, environment::Environment, filesystem::FileSystem, vault::Vault,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let fs = FileSystem::new(&env)?;
    let vault = Vault::new().await?;
    perform_backup(&env, &fs, &vault).await?;

    Ok(())
}
