use std::env;

pub struct Config {
    pub model: String,
    pub api_key: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let model = env::var("Q_MODEL")
            .unwrap_or_else(|_| "claude-haiku-4-5".into());

        let api_key = env::var("Q_ANTHROPIC_API_KEY")
            .map_err(|_| "Q_ANTHROPIC_API_KEY is not set.")?;

        Ok(Config { model, api_key })
    }
}
