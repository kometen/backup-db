#[cfg(test)]
mod tests {
    use std::env;

    use crate::compression::{
        check_compression_level_is_in_range, get_compression_level,
        get_compression_method_and_level,
    };
    use anyhow::Context;

    #[test]
    fn test_compression_method_is_none_when_env_value_is_missing() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::remove_var("COMPRESSION_METHOD");

        let compression_method_1 =
            env::var("COMPRESSION_METHOD").unwrap_or_else(|_| "".to_string());

        assert_eq!(
            get_compression_method_and_level(compression_method_1),
            ("none".to_string(), 0)
        );
    }

    #[test]
    fn test_compression_method_is_none_when_env_value_is_invalid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "foo");

        let compression_method_2 = env::var("COMPRESSION_METHOD")
            .context("Failed to get COMPRESSION_METHOD")
            .expect("COMPRESSION_METHOD should be set in test");

        assert_eq!(
            get_compression_method_and_level(compression_method_2),
            ("none".to_string(), 0)
        );
    }

    #[test]
    fn test_compression_level_is_default() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "gzip");

        let compression_method_3 = env::var("COMPRESSION_METHOD")
            .context("Failed to get COMPRESSION_METHOD")
            .expect("COMPRESSION_METHOD should be set in test");

        assert_eq!(
            get_compression_method_and_level(compression_method_3),
            ("gzip".to_string(), 6)
        );
    }

    #[test]
    fn test_compression_level_is_overriden() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "lz4");
        env::set_var("COMPRESSION_LEVEL", "4");

        let compression_method_and_level_4 = get_compression_method_and_level(
            env::var("COMPRESSION_METHOD")
                .context("Failed to get COMPRESSION_METHOD")
                .expect("COMPRESSION_METHOD should be set in test"),
        );

        assert_eq!(
            get_compression_level(
                env::var("COMPRESSION_LEVEL")
                    .context("Failed to get COMPRESSION_METHOD")
                    .expect("COMPRESSION_METHOD should be set in test"),
                compression_method_and_level_4.1
            )
            .unwrap(),
            4
        );
    }

    #[test]
    fn test_compression_level_is_not_in_range() {
        assert!(check_compression_level_is_in_range(10).is_err());
    }

    #[test]
    fn test_compression_level_is_in_range() {
        assert!(check_compression_level_is_in_range(5).is_ok());
    }
}
