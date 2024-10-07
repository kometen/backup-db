#[cfg(test)]
mod tests {
    use std::env;

    use crate::compression::{
        check_compression_level_is_in_range, get_compression_level,
        get_compression_method_and_level,
    };

    #[test]
    fn test_compression_method_is_none_when_env_value_is_missing() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        let compression_method = env::var("COMPRESSION_METHOD");
        assert_eq!(
            get_compression_method_and_level(compression_method),
            ("none".to_string(), 0)
        );
    }

    #[test]
    fn test_compression_method_is_none_when_env_value_is_invalid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "foo");
        let compression_method = env::var("COMPRESSION_METHOD");
        assert_eq!(
            get_compression_method_and_level(compression_method),
            ("none".to_string(), 0)
        );
    }

    #[test]
    fn test_compression_level_is_default() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "gzip");
        let compression_method = env::var("COMPRESSION_METHOD");
        assert_eq!(
            get_compression_method_and_level(compression_method),
            ("gzip".to_string(), 6)
        );
    }

    #[test]
    fn test_compression_level_is_overriden() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("COMPRESSION_METHOD", "lz4");
        env::set_var("COMPRESSION_LEVEL", "4");
        let compression_method_and_level =
            get_compression_method_and_level(env::var("COMPRESSION_METHOD"));
        assert_eq!(
            get_compression_level(
                env::var("COMPRESSION_LEVEL")
                    .map_err(|e| format!("Invalid COMPRESSION_LEVEL: {}", e)),
                compression_method_and_level.1
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
