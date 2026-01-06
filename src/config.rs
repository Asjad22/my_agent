use config::{Config, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub ollama: OllamaConfig,
    pub agent: AgentConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model: String,
    pub keep_alive: i64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AgentConfig {
    pub memory_window: usize,
}

impl AppConfig {
    pub fn load() -> Result<Self, config::ConfigError> {
        Config::builder()
            .add_source(File::with_name("config.toml"))
            .build()?
            .try_deserialize()
    }
}
