mod first_agent;
mod warmup_ollama;
use autoagents::llm::backends::ollama::Ollama;
use autoagents::llm::builder::LLMBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let llm: Arc<Ollama> = LLMBuilder::<Ollama>::new()
        .base_url("http://localhost:11434")
        .model("llama3.2:3b")
        // .model("gemma3:4b") // doesn't support tool call
        .build()?;

    // Warm up the model before using it
    warmup_ollama::warmup_ollama(&llm.model).await?;

    let out = first_agent::run_math_agent(llm).await?;
    println!("Agent output: {:?}", out);

    Ok(())
}
