#[cfg(test)]
mod tests {
    use crate::environment::get_buffer_size;
    use anyhow::Context;
    use dotenv;
    use std::env;

    /// Tests Environment returns a number or error.

    #[test]
    fn test_error_is_returned_when_value_is_not_a_number() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("BUFFER_SIZE", "value");

        let env_2 = env::var("BUFFER_SIZE")
            .context("Failed to get BUFFER_SIZE")
            .expect("BUFFER_SIZE should be set in test");
        let buffer_size_2 = get_buffer_size(env_2);

        assert!(buffer_size_2.is_err());
    }

    #[test]
    fn test_value_is_a_number() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("BUFFER_SIZE", "1967");

        let env_3 = env::var("BUFFER_SIZE")
            .context("Failed to get BUFFER_SIZE")
            .expect("BUFFER_SIZE should be set in test");
        let buffer_size_3 = get_buffer_size(env_3);

        assert_eq!(1967, buffer_size_3.unwrap());
    }

    #[test]
    fn test_default_value_when_variable_not_set() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::remove_var("BUFFER_SIZE");

        let env_4 = env::var("BUFFER_SIZE").unwrap_or_else(|_| "".to_string());
        let buffer_size_4 = get_buffer_size(env_4);

        assert!(buffer_size_4.is_err());
    }
}
