mod anthropic;
mod openai;

use crate::config::{Config, Provider};
use std::env;

pub fn call(config: &Config, query: &str) -> Result<String, String> {
    let system = system_prompt();
    match config.provider {
        Provider::Anthropic => anthropic::call(config, query, &system),
        Provider::OpenAI => openai::call(config, query, &system),
    }
}

fn command_text(text: String) -> Result<String, String> {
    if text.trim().is_empty() {
        Err("No text in response.".into())
    } else {
        Ok(text)
    }
}

fn system_prompt() -> String {
    let os = std::env::consts::OS;
    let shell = env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "sh".into());

    format!(
        "You translate natural language into shell commands. The user's OS is {os} and their shell is {shell}.\n\
         \n\
         Rules:\n\
         - Output ONLY the shell command. Nothing else.\n\
         - No explanations, no markdown, no backticks, no commentary, no prefixes.\n\
         - Your entire response will be passed directly to sh -c, so it must be a valid shell command.\n\
         - If the request doesn't map to a shell command, output the closest useful command.\n\
         - If there is truly no relevant command, output: echo \"No applicable command.\"\n\
         - For destructive operations (rm, drop, truncate, etc.), prefer safer variants \
         (e.g. rm -i, trash) unless the user's phrasing clearly indicates they want the forceful version."
    )
}
