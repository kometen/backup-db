mod tests;

use anyhow::{Context, Result};
use std::{env, process::Command};

pub struct SecretManager {
    pub url: String,
}

impl SecretManager {
    pub fn new() -> Result<Self> {
        Self::with_key("AZURE_KEY_VAULT_FAKTURA")
    }

    pub fn with_key(key: &str) -> Result<Self> {
        let azure_key_vault_url = env::var(&key).context(format!("Failed to get {}", key))?;

        let command = Command::new("op")
            .arg("read")
            .arg(&azure_key_vault_url)
            .output()
            .context("Error executing command")?;

        let url = String::from_utf8(command.stdout)
            .context("Failed to convert command output to string")?
            .trim_end()
            .to_string();

        Ok(Self { url })
    }

    #[cfg(test)]
    fn wrong_command_for_test() -> Result<Self> {
        let command = Command::new("_op_")
            .arg("read")
            .arg("foo")
            .output()
            .context("Error executing command")?;

        let url = String::from_utf8(command.stdout)
            .context("Failed to convert command output to string")?
            .trim_end()
            .to_string();

        Ok(Self { url })
    }
}
