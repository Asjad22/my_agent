use dotenv_rs::dotenv;
use std::env;

mod first_agent;

mod warmup_ollama;

use autoagents::llm::backends::ollama::Ollama;
use autoagents::llm::builder::LLMBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let base_url = env::var("OLLAMA_HOST_URL")?;
    let model = env::var("MODEL")?;
    let temperature: f32 = env::var("TEMPERATURE")?.parse()?;
    let keep_alive: i32 = env::var("KEEP_ALIVE")?.parse()?;
    let memory_window: usize = env::var("MEMORY_WINDOW")?.parse()?;

    let llm: Arc<Ollama> = LLMBuilder::<Ollama>::new()
        .base_url(base_url.clone())
        .model(model.clone())
        .temperature(temperature)
        // .model("gemma3:4b") // doesn't support tool call
        .build()?;

    // Warm up the model before using it
    warmup_ollama::warmup_ollama(&base_url, &model, keep_alive).await?;

    let out = first_agent::run_math_agent(llm, memory_window).await?;
    println!("Agent output: {:?}", out);

    Ok(())
}
