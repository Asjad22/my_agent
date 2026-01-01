mod first_agent;
use autoagents::llm::backends::ollama::Ollama;
use autoagents::llm::builder::LLMBuilder;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let llm: Arc<Ollama> = LLMBuilder::<Ollama>::new()
        .base_url("http://localhost:11434")
        .model("llama3.2:3b")
        .build()?;

    let out = first_agent::run_math_agent(llm).await?;
    println!("Agent output: {:?}", out);

    Ok(())
}
