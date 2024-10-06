#[cfg(test)]
mod tests {
    use crate::environment::get_buffer_size;
    use dotenv;
    use std::env;

    #[test]
    fn test_error_is_returned_when_value_is_missing() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        let env = env::var("BUFFER_SIZE");
        let buffer_size = get_buffer_size(env);
        assert!(buffer_size.is_err());
    }

    #[test]
    fn test_error_is_returned_when_value_is_not_a_number() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("BUFFER_SIZE", "value");
        let env = env::var("BUFFER_SIZE");
        let buffer_size = get_buffer_size(env);
        assert!(buffer_size.is_err());
    }

    #[test]
    fn test_value_is_a_number() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("BUFFER_SIZE", "1967");
        let env = env::var("BUFFER_SIZE");
        let buffer_size = get_buffer_size(env);
        assert_eq!(1967, buffer_size.unwrap());
    }

    #[test]
    fn test_default_value_when_variable_not_set() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::remove_var("BUFFER_SIZE");
        let env = env::var("BUFFER_SIZE");
        let buffer_size = get_buffer_size(env);
        assert_eq!(8192, buffer_size.unwrap());
    }
}
