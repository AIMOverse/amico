mod agent;
mod core;
mod error;
mod event;
mod interface;
mod params;
mod provider;
mod runtime;

pub use core::CoreConfig;
pub use error::ConfigError;
pub use interface::*;
pub use params::*;
pub use provider::*;
pub use runtime::*;

#[cfg(test)]
mod tests {
    use super::*;

    const TESTS_DIR: &str = "tests/config";

    #[test]
    fn parse_config() {
        let cwd = std::env::current_dir().expect("Failed to get current working directory");
        assert!(cwd.to_str().unwrap().ends_with("amico-core"));

        let config_path = format!("{}/example.toml", TESTS_DIR);
        let config_str = std::fs::read_to_string(&config_path).unwrap();
        let config = CoreConfig::from_toml_str(&config_str).unwrap();

        // Test version
        assert_eq!(config.version, 0);

        // Test runtime
        assert_eq!(config.runtime, RuntimeConfig::Standalone);

        // Test agents
        assert_eq!(config.agents.len(), 1);
        let agent = &config.agents[0];
        assert_eq!(agent.name, "AmIco");
        assert_eq!(
            agent.system_prompt,
            "You are AmIco, an AI that helps people."
        );
        assert_eq!(agent.provider, "openai");
        assert_eq!(agent.model, "gpt-4o-mini");
        assert_eq!(agent.temperature, Some(0.7));
        assert_eq!(agent.max_tokens, Some(1000));

        // Test providers
        let openai = &config.providers.openai;
        assert_eq!(openai.base_url, "https://api.openai.com/v1");
        assert_eq!(openai.api_key, "sk-...");

        // Test custom providers
        assert_eq!(config.providers.custom.len(), 2);
        let local = &config.providers.custom[0];
        assert_eq!(local.name, "local");
        assert_eq!(local.schema, ApiSchema::Openai);
        assert_eq!(local.base_url, "http://localhost:8000");
        assert!(local.api_key.is_none());

        let example = &config.providers.custom[1];
        assert_eq!(example.name, "example");
        assert_eq!(example.schema, ApiSchema::Openai);
        assert_eq!(example.base_url, "http://example.com");
        assert_eq!(example.api_key.as_deref(), Some("sk-..."));

        // Test events
        assert_eq!(config.events.len(), 2);
        let event = &config.events[0];
        assert_eq!(event.name, "interval_10min");
        assert_eq!(event.source, "interval");
        let params = event.params.as_ref().unwrap();
        assert_eq!(params.len(), 1);
        assert_eq!(event.param("mins").unwrap(), &ParamValue::Integer(10));
    }
}
