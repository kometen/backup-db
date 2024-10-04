use backup_db::{backup::backup::perform_backup, environment::Environment, vault::Vault};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let vault = Vault::new().await?;
    perform_backup(&env, &vault).await?;

    Ok(())
}
