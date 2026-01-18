// config.rs
use anyhow::Context;
use std::env;

/// Ollama LLM configuration
#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub base_url: String,
    pub model: String,
    pub keep_alive: i32,
    pub temperature: f32,
}

/// Agent-specific configuration
#[derive(Debug, Clone)]
pub struct AgentConfig {
    pub memory_window: usize,
}

/// Full app configuration
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub ollama: OllamaConfig,
    pub agent: AgentConfig,
}

impl AppConfig {
    /// Build AppConfig from environment variables (.env)
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv_rs::dotenv().ok(); // load .env

        // Helper closure to parse env vars
        let parse_env = |key: &str| -> anyhow::Result<String> {
            env::var(key).with_context(|| format!("Missing environment variable {}", key))
        };

        // Ollama config
        let base_url = parse_env("OLLAMA_HOST_URL")?;
        let model = parse_env("MODEL")?;
        let temperature: f32 = parse_env("TEMPERATURE")?
            .parse()
            .with_context(|| "Failed to parse TEMPERATURE as f32")?;
        let keep_alive: i32 = parse_env("KEEP_ALIVE")?
            .parse()
            .with_context(|| "Failed to parse KEEP_ALIVE as i32")?;

        // Agent config
        let memory_window: usize = parse_env("MEMORY_WINDOW")?
            .parse()
            .with_context(|| "Failed to parse MEMORY_WINDOW as usize")?;

        Ok(Self {
            ollama: OllamaConfig {
                base_url,
                model,
                keep_alive,
                temperature,
            },
            agent: AgentConfig { memory_window },
        })
    }
}
