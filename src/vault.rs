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

        let db_host = get_secret(&client, String::from("db-host")).await;
        let db_user = get_secret(&client, String::from("db-user")).await;
        let db_name = get_secret(&client, String::from("db-name")).await;
        let db_pwd = get_secret(&client, String::from("db-pwd")).await;

        let connect_string = format!(
            "postgres://{}:{}@{}.{}/{}",
            db_user, db_pwd, db_host, domain, db_name
        );

        Ok(Self { connect_string })
    }
}

async fn get_secret(client: &SecretClient, key: String) -> String {
    client
        .get(key.clone())
        .await
        .map_err(|e| format!("Error fetching secret using key {}: {}", key, e))
        .unwrap()
        .value
}
