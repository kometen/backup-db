# backup_db
This Rust-program will get secrets from Azure Key Vault to get credentials to a PostgreSQL database and perform a backup using pg_dump. Log in to Azure with `az login`.

Some settings can be set in a `.env` file like this:

```
BUFFER_SIZE=16384
COMPRESSION_METHOD=lz4
DOMAIN=svc.cluster.local
FILE_PREFIX=production
FOLDER=backup
KEYVAULT_URL=https://foo-bar-baz.vault.azure.net/
```

Based on an example from https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/security_keyvault/examples/get_secret.rs and guidance from both ChatGPT and Claude.
