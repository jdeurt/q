use crate::config::Config;
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    content: Vec<ContentBlock>,
    stop_reason: String,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum ContentBlock {
    #[serde(rename = "text")]
    Text { text: String },
    #[serde(other)]
    Other,
}

fn request_body(config: &Config, query: &str, system: &str) -> serde_json::Value {
    let mut body = serde_json::json!({
        "model": config.model,
        "max_tokens": 16384,
        "system": system,
        "messages": [
            { "role": "user", "content": query }
        ]
    });
    if let Some(effort) = &config.effort {
        body["output_config"] = serde_json::json!({ "effort": effort });
    }
    body
}

pub fn call(config: &Config, query: &str, system: &str) -> Result<String, String> {
    let mut resp = ureq::post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", &config.api_key)
        .header("anthropic-version", "2023-06-01")
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
        if self.stop_reason != "end_turn" {
            return Err(format!(
                "Anthropic response did not complete: {}.",
                self.stop_reason
            ));
        }
        super::command_text(
            self.content
                .into_iter()
                .filter_map(|block| match block {
                    ContentBlock::Text { text } => Some(text),
                    ContentBlock::Other => None,
                })
                .collect(),
        )
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
            provider: Provider::Anthropic,
            model: "claude-haiku-5-5".into(),
            api_key: "test".into(),
            effort: None,
        };
        let body = request_body(&config, "list files", "system prompt");
        assert!(body.get("output_config").is_none());
        assert_eq!(body["messages"][0]["content"], "list files");
        assert_eq!(body["system"], "system prompt");
        config.effort = Some("low".into());
        assert_eq!(
            request_body(&config, "query", "system")["output_config"],
            json!({"effort": "low"})
        );
    }

    #[test]
    fn extracts_text_after_thinking() {
        let response: Response = serde_json::from_value(json!({"stop_reason": "end_turn", "content": [{"type": "thinking", "thinking": "reasoning", "signature": "sig"}, {"type": "text", "text": "ls -la"}]})).unwrap();
        assert_eq!(response.command().unwrap(), "ls -la");
    }

    #[test]
    fn rejects_incomplete_refused_or_empty_responses() {
        for reason in ["max_tokens", "refusal", "tool_use"] {
            let response: Response = serde_json::from_value(json!({"stop_reason": reason, "content": [{"type": "text", "text": "partial command"}]})).unwrap();
            assert!(response.command().is_err());
        }
        for content in [
            json!([]),
            json!([{"type": "thinking"}]),
            json!([{"type": "text", "text": " "}]),
        ] {
            let response: Response =
                serde_json::from_value(json!({"stop_reason": "end_turn", "content": content}))
                    .unwrap();
            assert!(response.command().is_err());
        }
    }
}
