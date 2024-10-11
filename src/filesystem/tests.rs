#[cfg(test)]
mod tests {
    use dirs::home_dir;
    use std::env;

    use crate::filesystem::check_folder;

    #[test]
    fn test_path_is_valid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("FOLDER", ".");
        let folder_1 = env::var("FOLDER").unwrap();
        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());
        let path_1 = check_folder(&home, &folder_1.as_str());
        assert!(path_1.is_ok());
    }

    #[test]
    fn test_path_is_invalid() {
        dotenv::from_path("./src/data/.env.test").unwrap();
        env::set_var("FOLDER", "foo-bar-baz");
        let folder_2 = env::var("FOLDER").unwrap();
        let home = home_dir().unwrap_or_else(|| "".parse().unwrap());
        let path_2 = check_folder(&home, &folder_2.as_str());
        assert!(path_2.is_err());
    }
}
