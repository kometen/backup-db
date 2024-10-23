# backup_db
This Rust-program will perform a backup using pg_dump.

It gets an URL to an Azure Key Vault from 1password and then retrieve secrets
from Azure Key Vault to a PostgreSQL database and performs the backup.

Log in to Azure with `az login`.

Some settings can be set in a `.env` file like this:

```
BUFFER_SIZE=16384
COMPRESSION_METHOD=lz4
DOMAIN=svc.cluster.local
FILE_PREFIX=production
FOLDER=backup
```

Clone the repository and build with `cargo build [--release]`. Or build a container-image
with `docker build -t backup_db:dev .`. This does not currently inherit the Azure Login
environment so will not work without modifications.

Test with `cargo test`.

Requires 1password command line utilities installed locally, an Azure-subscription, a PostgreSQL database.

Based on an example from https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/security_keyvault/examples/get_secret.rs and guidance from both ChatGPT and Claude.
