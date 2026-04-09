use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
}

pub fn call(config: &Config, query: &str, system: &str) -> Result<String, String> {
    let body = serde_json::json!({
        "model": config.model,
        "max_tokens": 1024,
        "system": system,
        "messages": [
            { "role": "user", "content": query }
        ]
    });

    let mut resp = ureq::post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .send_json(&body)
        .map_err(|e| format!("API request failed: {e}"))?;

    let response: Response = resp
        .body_mut()
        .read_json()
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    response
        .content
        .into_iter()
        .find_map(|block| match block {
            ContentBlock::Text { text } => Some(text),
        })
        .ok_or_else(|| "No text in response.".to_string())
}
