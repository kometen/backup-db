pub mod dns {

    use crate::{Environment, Vault};

    pub fn check_dns(vault: &Vault, env: &Environment) -> Result<(), Box<dyn std::error::Error>> {
        use trust_dns_resolver::{
            config::{ResolverConfig, ResolverOpts},
            AsyncResolver,
        };

        let hostname = format!("{}.{}", &vault.host, &env.domain);
        let resolver = AsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
        Ok(())
    }
}
