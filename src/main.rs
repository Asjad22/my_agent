mod config;
mod first_agent;
mod warmup_ollama;
use autoagents::llm::backends::ollama::Ollama;
use autoagents::llm::builder::LLMBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cfg = config::AppConfig::load()?;

    let llm: Arc<Ollama> = LLMBuilder::<Ollama>::new()
        .base_url(&cfg.ollama.base_url)
        .model(&cfg.ollama.model)
        // .model("gemma3:4b") // doesn't support tool call
        .build()?;

    // Warm up the model before using it
    warmup_ollama::warmup_ollama(&cfg).await?;

    let out = first_agent::run_math_agent(llm, &cfg).await?;
    println!("Agent output: {:?}", out);

    Ok(())
}
