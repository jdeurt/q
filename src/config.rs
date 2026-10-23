use std::env;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Provider {
    Anthropic,
    OpenAI,
}

impl Provider {
    fn default_model(self) -> &'static str {
        match self {
            Self::Anthropic => "claude-haiku-5-5",
            Self::OpenAI => "gpt-6-luna",
        }
    }

    fn api_key_variable(self) -> &'static str {
        match self {
            Self::Anthropic => "Q_ANTHROPIC_API_KEY",
            Self::OpenAI => "Q_OPENAI_API_KEY",
        }
    }

    fn efforts(self) -> &'static [&'static str] {
        match self {
            Self::Anthropic => &["low", "medium", "high", "xhigh", "max"],
            Self::OpenAI => &["none", "minimal", "low", "medium", "high", "xhigh", "max"],
        }
    }
}

pub struct Config {
    pub provider: Provider,
    pub model: String,
    pub api_key: String,
    pub effort: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Self::from_lookup(|name| env::var(name).ok())
    }

    fn from_lookup(get: impl Fn(&str) -> Option<String>) -> Result<Self, String> {
        let provider = match get("Q_PROVIDER").as_deref().unwrap_or("anthropic") {
            "anthropic" => Provider::Anthropic,
            "openai" => Provider::OpenAI,
            value => {
                return Err(format!(
                    "Invalid Q_PROVIDER {value:?}. Use anthropic or openai."
                ))
            }
        };
        let model = get("Q_MODEL").unwrap_or_else(|| provider.default_model().into());
        let effort = get("Q_EFFORT").filter(|value| !value.trim().is_empty());
        if let Some(value) = &effort {
            if !provider.efforts().contains(&value.as_str()) {
                return Err(format!(
                    "Invalid Q_EFFORT {value:?}. For this provider, use: {}.",
                    provider.efforts().join(", ")
                ));
            }
        }
        let key_variable = provider.api_key_variable();
        let api_key = get(key_variable)
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| format!("{key_variable} is not set."))?;

        Ok(Config {
            provider,
            model,
            api_key,
            effort,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn config(vars: &[(&str, &str)]) -> Result<Config, String> {
        Config::from_lookup(|name| {
            vars.iter()
                .find(|(key, _)| *key == name)
                .map(|(_, value)| value.to_string())
        })
    }

    #[test]
    fn defaults_to_haiku_without_effort() {
        let config = config(&[("Q_ANTHROPIC_API_KEY", "test")]).unwrap();
        assert_eq!(config.provider, Provider::Anthropic);
        assert_eq!(config.model, "claude-haiku-5-5");
        assert!(config.effort.is_none());
    }

    #[test]
    fn openai_uses_its_own_key_and_default_model() {
        let config = config(&[
            ("Q_PROVIDER", "openai"),
            ("Q_OPENAI_API_KEY", "openai-test"),
            ("Q_ANTHROPIC_API_KEY", "anthropic-test"),
        ])
        .unwrap();
        assert_eq!(config.provider, Provider::OpenAI);
        assert_eq!(config.model, "gpt-6-luna");
        assert_eq!(config.api_key, "openai-test");
        assert!(config.effort.is_none());
    }

    #[test]
    fn preserves_model_and_effort_overrides_for_both_providers() {
        for provider in ["anthropic", "openai"] {
            let config = config(&[
                ("Q_PROVIDER", provider),
                ("Q_MODEL", "custom-model"),
                ("Q_EFFORT", "high"),
                ("Q_ANTHROPIC_API_KEY", "test"),
                ("Q_OPENAI_API_KEY", "test"),
            ])
            .unwrap();
            assert_eq!(config.model, "custom-model");
            assert_eq!(config.effort.as_deref(), Some("high"));
        }
    }

    #[test]
    fn blank_effort_is_unspecified() {
        let config = config(&[("Q_ANTHROPIC_API_KEY", "test"), ("Q_EFFORT", "  ")]).unwrap();
        assert!(config.effort.is_none());
    }

    #[test]
    fn rejects_invalid_provider_and_effort() {
        assert!(config(&[("Q_PROVIDER", "other")])
            .err()
            .unwrap()
            .contains("Q_PROVIDER"));
        for (provider, effort) in [("anthropic", "none"), ("openai", "invalid")] {
            assert!(config(&[("Q_PROVIDER", provider), ("Q_EFFORT", effort)])
                .err()
                .unwrap()
                .contains("Q_EFFORT"));
        }
    }

    #[test]
    fn requires_the_selected_providers_nonempty_key() {
        for value in [None, Some(""), Some("  ")] {
            let mut vars = vec![("Q_PROVIDER", "openai"), ("Q_ANTHROPIC_API_KEY", "test")];
            if let Some(value) = value {
                vars.push(("Q_OPENAI_API_KEY", value));
            }
            assert_eq!(config(&vars).err().unwrap(), "Q_OPENAI_API_KEY is not set.");
        }
        assert_eq!(
            config(&[]).err().unwrap(),
            "Q_ANTHROPIC_API_KEY is not set."
        );
    }
}
