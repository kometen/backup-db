pub mod dns {

    use crate::{Environment, Vault};
    use hickory_resolver::error::ResolveError;
    use hickory_resolver::system_conf::read_system_conf;
    use hickory_resolver::AsyncResolver;

    pub async fn check_dns(
        vault: &Vault,
        env: &Environment,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (config, opts) = read_system_conf().map_err(|e| ResolveError::from(e))?;
        let resolver = AsyncResolver::tokio(config, opts);
        let hostname = format!("{}.{}.", &vault.host, &env.domain);
        let response = resolver.lookup_ip(hostname).await?;

        if response.iter().next().is_none() {
            return Err("No addresses returned".into());
        }

        Ok(())
    }
}
