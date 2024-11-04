pub mod backup;
pub mod compression;
pub mod dns;
pub mod environment;
pub mod filesystem;
pub mod secret_manager;

pub use backup::backup::perform_backup;
pub use compression::Compression;
pub use dns::dns::check_dns;
pub use environment::Environment;
pub use filesystem::FileSystem;
pub use secret_manager::SecretManager;

use azure_vault_secrets::VaultStorage;
use db_config::db_config_from_vault;

db_config_from_vault!([host, user, name, pwd, domain]);
