use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    status: String,
    output: Vec<OutputItem>,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum OutputItem {
    #[serde(rename = "message")]
    Message { content: Vec<ContentBlock> },
    #[serde(other)]
    Other,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "output_text")]
    Text { text: String },
    #[serde(rename = "refusal")]
    Refusal,
    #[serde(other)]
    Other,
}

fn request_body(config: &Config, query: &str, system: &str) -> serde_json::Value {
    let mut body = serde_json::json!({
        "model": config.model,
        "instructions": system,
        "input": [{ "role": "user", "content": query }],
        "max_output_tokens": 16384,
        "store": false,
    });
    if let Some(effort) = &config.effort {
        body["reasoning"] = serde_json::json!({ "effort": effort });
    }
    body
}

pub fn call(config: &Config, query: &str, system: &str) -> Result<String, String> {
    let mut resp = ureq::post("https://api.openai.com/v1/responses")
        .header("Authorization", &format!("Bearer {}", config.api_key))
        .header("content-type", "application/json")
        .send_json(request_body(config, query, system))
        .map_err(|e| format!("API request failed: {e}"))?;

    let response: Response = resp
        .body_mut()
        .read_json()
        .map_err(|e| format!("Failed to parse response: {e}"))?;
    response.command()
}

impl Response {
    fn command(self) -> Result<String, String> {
        if self.status != "completed" {
            return Err(format!(
                "OpenAI response did not complete: {}.",
                self.status
            ));
        }
        let mut text = String::new();
        for item in self.output {
            if let OutputItem::Message { content } = item {
                for block in content {
                    match block {
                        ContentBlock::Text { text: part } => text.push_str(&part),
                        ContentBlock::Refusal => return Err("OpenAI refused the request.".into()),
                        ContentBlock::Other => {}
                    }
                }
            }
        }
        super::command_text(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Provider;
    use serde_json::json;

    #[test]
    fn request_omits_unspecified_effort_and_maps_explicit_effort() {
        let mut config = Config {
            provider: Provider::OpenAI,
            model: "gpt-6-luna".into(),
            api_key: "test".into(),
            effort: None,
        };
        let body = request_body(&config, "list files", "system prompt");
        assert!(body.get("reasoning").is_none());
        assert_eq!(body["model"], "gpt-6-luna");
        assert_eq!(body["instructions"], "system prompt");
        assert_eq!(body["input"][0]["content"], "list files");
        assert_eq!(body["store"], false);
        config.effort = Some("none".into());
        assert_eq!(
            request_body(&config, "query", "system")["reasoning"],
            json!({"effort": "none"})
        );
    }

    #[test]
    fn extracts_command_after_reasoning_items() {
        let response: Response = serde_json::from_value(json!({"status": "completed", "output": [{"type": "reasoning", "summary": []}, {"type": "message", "content": [{"type": "output_text", "text": "ls -la", "annotations": []}]}]})).unwrap();
        assert_eq!(response.command().unwrap(), "ls -la");
    }

    #[test]
    fn rejects_incomplete_or_failed_responses() {
        for status in ["incomplete", "failed", "in_progress"] {
            let response: Response = serde_json::from_value(json!({"status": status, "output": [{"type": "message", "content": [{"type": "output_text", "text": "partial command"}]}]})).unwrap();
            assert!(response.command().is_err());
        }
    }

    #[test]
    fn rejects_refusals_and_missing_command_text() {
        for content in [
            json!([]),
            json!([{"type": "refusal", "refusal": "Cannot help"}]),
            json!([{"type": "output_text", "text": " "}]),
            json!([{"type": "output_text", "text": "partial"}, {"type": "refusal", "refusal": "Cannot help"}]),
        ] {
            let response: Response = serde_json::from_value(
                json!({"status": "completed", "output": [{"type": "message", "content": content}]}),
            )
            .unwrap();
            assert!(response.command().is_err());
        }
        let response: Response = serde_json::from_value(
            json!({"status": "completed", "output": [{"type": "reasoning", "summary": []}]}),
        )
        .unwrap();
        assert!(response.command().is_err());
    }
}
