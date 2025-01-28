mod core_config;
mod runtime;

pub use core_config::*;
pub use runtime::*;

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;

    const TESTS_DIR: &str = "tests/config";

    fn load_config(filename: &str) -> CoreConfig {
        let cwd = std::env::current_dir().expect("Failed to get current working directory");
        assert!(cwd.to_str().unwrap().ends_with("amico-core"));
        let config_path = format!("{}/{}", TESTS_DIR, filename);
        let config_str = std::fs::read_to_string(&config_path).unwrap();
        CoreConfig::from_toml_str(&config_str).unwrap()
    }

    #[test]
    fn parse_example_config() {
        let config = load_config("example.toml");

        // Test name
        assert_eq!(config.name, "AIMO");

        // Test version
        assert_eq!(config.version, 0);

        // Test runtime
        assert_eq!(config.runtime, RuntimeConfig::Standalone);
    }
}
