#[cfg(test)]
mod tests {
    use super::super::SecretManager;
    use std::env;

    #[test]
    fn test_new_with_valid_env_var() {
        env::set_var(
            "AZURE_KEY_VAULT_TEST",
            "op://Production/AzureKeyVaultTest/credentials/url",
        );

        if env::var("GITHUB_ACTIONS").is_ok() {
            let result = env::var("AZURE_KEY_VAULT_TEST");
            assert!(result.is_ok());
            let secret_manager = result.unwrap();
            assert_eq!(secret_manager, "https://foo.bar.baz.net/");
        } else {
            let result = SecretManager::with_key("AZURE_KEY_VAULT_TEST");
            assert!(result.is_ok());
            let secret_manager = result.unwrap();
            assert_eq!(secret_manager.url, "https://foo.bar.baz.net/");
        }
    }

    #[test]
    fn test_new_with_missing_env_var() {
        env::remove_var("AZURE_KEY_VAULT_FAKTURA");
        let result = SecretManager::with_key("AZURE_KEY_VAULT_FAKTURA");
        assert!(result.is_err());
    }

    #[test]
    fn test_new_with_invalid_command() {
        let result = SecretManager::wrong_command_for_test();
        assert!(result.is_err());
    }
}
