//! Vault Module
//!
//! This module retrieve secrets from Azure Key Vault URL.
//!

use anyhow::{Context, Result};
use azure_security_keyvault::SecretClient;

pub struct Vault {
    pub host: String,
    pub user: String,
    pub name: String,
    pub pwd: String,
    pub domain: String,
}

impl Vault {
    /// Creates a new Secret Manager instance with a specific value.
    ///
    /// # Arguments
    ///
    /// * `url` - Name of the variable with the URL to Azure Key Vault
    ///
    /// # Returns
    ///
    /// A Result containing the Vault if successful, or an error if the secret
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use backup_db::Vault;
    /// use anyhow::Result;
    ///
    /// async fn example() -> Result<()> {
    ///     let vault = Vault::new("URL".to_string()).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn new(url: String) -> Result<Self> {
        let credential =
            azure_identity::create_credential().context("Unable to create an Azure Identity")?;
        let client = SecretClient::new(url.as_str(), credential)?;

        let host = get_secret(&client, String::from("db-host")).await?;
        let user = get_secret(&client, String::from("db-user")).await?;
        let name = get_secret(&client, String::from("db-name")).await?;
        let pwd = get_secret(&client, String::from("db-pwd")).await?;
        let domain = get_secret(&client, String::from("db-domain")).await?;

        Ok(Self {
            host,
            user,
            name,
            pwd,
            domain,
        })
    }
}

async fn get_secret(client: &SecretClient, key: String) -> Result<String> {
    let response = client.get(key).await.context("Unable to retrieve value")?;
    Ok(response.value.to_string())
}
