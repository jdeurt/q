mod anthropic;

use crate::config::Config;
use std::env;

pub fn call(config: &Config, query: &str) -> Result<String, String> {
    let system = system_prompt();
    anthropic::call(config, query, &system)
}

fn system_prompt() -> String {
    let os = std::env::consts::OS;
    let shell = env::var("SHELL")
        .ok()
        .and_then(|s| s.rsplit('/').next().map(String::from))
        .unwrap_or_else(|| "sh".into());

    format!(
        "You are a command-line assistant. The user's OS is {os} and their shell is {shell}.\n\
         \n\
         Respond with ONLY the raw shell command — no explanation, no markdown, no backticks, no commentary.\n\
         \n\
         For destructive operations (rm, drop, truncate, etc.), prefer safer variants \
         (e.g. rm -i, trash) unless the user's phrasing clearly indicates they want the forceful version."
    )
}
