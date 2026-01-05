use reqwest::Client;
use serde_json::json;

// Check if the Ollama is already loaded
pub async fn is_model_loaded(model: &str) -> anyhow::Result<bool> {
    let client = Client::new();
    let resp = client.get("http://localhost:11434/api/ps").send().await?;

    if !resp.status().is_success() {
        return Ok(false);
    }

    let json_resp: serde_json::Value = resp.json().await?;
    if let Some(models) = json_resp.get("models").and_then(|m| m.as_array()) {
        for m in models {
            if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                if name == model {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}

// Warm up Ollama model so it stays loaded
pub async fn warmup_ollama(model: &str) -> anyhow::Result<()> {
    // only run warm-up if not loaded
    if is_model_loaded(model).await? {
        println!(
            "Ollama mdoel '{}' is already loaded, skipping warmup",
            model,
        );
        return Ok(());
    }

    let client = Client::new();
    let resp = client
        .post("http://localhost::11434/api/chat")
        .json(&json!({
            "model": model,
            "messages": [
                { "role": "user", "content": "warm up" }
            ],
            "keep_alive": -1  // or "10m" if you prefer
        }))
        .send()
        .await?;

    if !resp.status().is_success() {
        let body = resp.text().await?;
        eprint!("Ollama warmpup failed: {}", body);
    } else {
        println!("Ollama model '{}' warmed up", model);
    }
    Ok(())
}
