#[cfg(test)]
mod tests {
    use dirs::home_dir;
    use std::env;

    use crate::filesystem::check_folder;

    #[test]
    fn test_path_is_valid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("FOLDER", ".");
        let folder = env::var("FOLDER").unwrap();
        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());
        let path = check_folder(&home, &folder.as_str());
        assert!(path.is_ok());
    }

    #[test]
    fn test_path_is_invalid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("FOLDER", "foo-bar-baz");
        let folder = env::var("FOLDER").unwrap();
        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());
        let path = check_folder(&home, &folder.as_str());
        assert!(path.is_err());
    }
}
