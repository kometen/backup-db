use azure_security_keyvault::SecretClient;
use std::env;

pub struct Vault {
    pub connect_string: String,
}

impl Vault {
    pub async fn new() -> Result<Self, std::io::Error> {
        let keyvault_url =
            env::var("KEYVAULT_URL").expect("Missing KEYVAULT_URL environment variable.");
        let credential = azure_identity::create_credential().unwrap();
        let client = SecretClient::new(keyvault_url.as_str(), credential).unwrap();
        let domain = env::var("DOMAIN").expect("Missing DOMAIN environment variable.");

        let db_host = client
            .get("db-host")
            .await
            .map_err(|e| format!("Error fetching db-host: {}", e))
            .unwrap()
            .value;

        let db_user = client
            .get("db-user")
            .await
            .map_err(|e| format!("Error fetching db-user: {}", e))
            .unwrap()
            .value;

        let db_name = client
            .get("db-name")
            .await
            .map_err(|e| format!("Error fetching db-name: {}", e))
            .unwrap()
            .value;

        let db_pwd = client
            .get("db-pwd")
            .await
            .map_err(|e| format!("Error fetching db-pwd: {}", e))
            .unwrap()
            .value;

        let connect_string = format!(
            "postgres://{}:{}@{}.{}/{}",
            db_user, db_pwd, db_host, domain, db_name
        );

        Ok(Self { connect_string })
    }
}
