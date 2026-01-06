use crate::config::AppConfig;
use autoagents::async_trait;
use autoagents::core::agent::prebuilt::executor::{ReActAgent, ReActAgentOutput};
use autoagents::core::tool::{ToolCallError, ToolRuntime};
use autoagents::llm::LLMProvider;
use autoagents::prelude::{
    AgentBuilder, AgentOutputT, DirectAgent, SlidingWindowMemory, Task, ToolInputT, ToolT,
};
use autoagents_derive::{AgentHooks, AgentOutput, agent, tool};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// Tool input
#[derive(Serialize, Deserialize, Debug)]
struct AddArgs {
    left: i64,
    right: i64,
}

/// Implement ToolInputT manually with io_schema
impl ToolInputT for AddArgs {
    fn io_schema() -> &'static str {
        r#"
        {
            "type": "object",
            "properties": {
                "left": { "type": "integer" },
                "right": { "type": "integer" }
            },
            "required": ["left", "right"]
        }
        "#
    }
}

/// Tool implementation
#[tool(name = "addition", description = "Add two numbers", input = AddArgs)]
struct Addition;

#[async_trait]
impl ToolRuntime for Addition {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let a: AddArgs = serde_json::from_value(args)?;
        Ok((a.left + a.right).into())
    }
}

/// Agent output
#[derive(Debug, Serialize, Deserialize, AgentOutput)]
pub struct MathOut {
    #[output(description = "The result value")]
    value: i64,
    #[output(description = "Short explanation")]
    explanation: String,
}

/// Agent definition
#[agent(
    name = "math_agent",
    description = "Solve basic math using tools and return JSON",
    tools = [Addition],
    output = MathOut
)]
#[derive(Clone, AgentHooks, Default)]
struct MathAgent;

impl From<ReActAgentOutput> for MathOut {
    fn from(out: ReActAgentOutput) -> Self {
        serde_json::from_str(&out.response).unwrap_or(MathOut {
            value: 0,
            explanation: out.response,
        })
    }
}

/// Run agent
pub async fn run_math_agent(llm: Arc<dyn LLMProvider>, cfg: &AppConfig) -> anyhow::Result<MathOut> {
    let memory = Box::new(SlidingWindowMemory::new(cfg.agent.memory_window));
    let agent = ReActAgent::new(MathAgent::default());

    let handle = AgentBuilder::<_, DirectAgent>::new(agent)
        .llm(llm)
        .memory(memory)
        .build()
        .await?;

    let result = handle
        .agent
        .run(Task::new("Add 20 and 5 and explain why?"))
        .await?;

    Ok(result)
}
