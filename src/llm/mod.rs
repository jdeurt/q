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
