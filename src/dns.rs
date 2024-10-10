pub mod dns {

    use crate::{Environment, Vault};
    use anyhow::{Context, Result};
    use hickory_resolver::error::ResolveError;
    use hickory_resolver::system_conf::read_system_conf;
    use hickory_resolver::AsyncResolver;

    pub async fn check_dns(vault: &Vault, env: &Environment) -> Result<()> {
        let (config, opts) = read_system_conf().map_err(|e| ResolveError::from(e))?;
        let resolver = AsyncResolver::tokio(config, opts);
        let hostname = format!("{}.{}.", &vault.host, &env.domain);
        let response = resolver
            .lookup_ip(&hostname)
            .await
            .with_context(|| format!("Failed to resolve hostname: {}", hostname))?;

        if response.iter().next().is_none() {
            anyhow::bail!("No IP address found for hostname: {}", hostname);
        }

        Ok(())
    }
}
