mod tests;
use anyhow::{Context, Result};
use dotenv::dotenv;
use std::env;

pub struct Environment {
    pub buffer_size: usize,
}

impl Environment {
    /// Creates a new Environment instance with a specific value.
    ///
    /// # Returns
    ///
    /// A Result containing the Environment if successful, or an error if the secret
    /// could not be retrieved.
    ///
    /// # Example
    ///
    /// ```
    /// use backup_db::Environment;
    /// use anyhow::Result;
    ///
    /// fn example() -> Result<()> {
    ///     let environment = Environment::new()?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new() -> Result<Self> {
        dotenv().ok();

        let buffer_size = env::var("BUFFER_SIZE")
            .map(|s| get_buffer_size(s))
            .unwrap_or(Ok(8192))?;

        Ok(Self { buffer_size })
    }
}

fn get_buffer_size(env: String) -> Result<usize> {
    let buffer_size = env
        .parse::<usize>()
        .context(format!("Invalid BUFFER_SIZE value: {}", env))?;

    Ok(buffer_size)
}
