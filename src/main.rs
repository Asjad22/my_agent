use autoagents::core::agent::prebuilt::executor::BasicAgent;
use autoagents::core::agent::task::Task;
use autoagents::core::agent::{AgentBuilder, DirectAgent};
use autoagents::llm::backends::ollama::Ollama;
use autoagents::llm::builder::LLMBuilder;
use autoagents_derive::{AgentHooks, agent};
use std::sync::Arc;

#[agent(name = "hello", description = "Helpful assistant")]
#[derive(Clone, AgentHooks, Default)]
struct HelloAgent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Set OPENAI_API_KEY in your env
    let llm: Arc<Ollama> = LLMBuilder::<Ollama>::new()
        .base_url("http://localhost:11434")
        .model("llama3.2:3b")
        .build()?;

    let agent = BasicAgent::new(HelloAgent);
    let handle = AgentBuilder::<_, DirectAgent>::new(agent)
        .llm(llm)
        .build()
        .await?;

    let out = handle
        .agent
        .run(Task::new("Say hi in one short sentence"))
        .await?;
    println!("{}", String::from(out));
    Ok(())
}
