use std::env;
use std::fs;
use std::path::PathBuf;

pub fn load() -> Result<(), String> {
    let shell = detect_shell()?;
    print!("{}", shell_snippet(&shell));
    Ok(())
}

pub fn init() -> Result<(), String> {
    let shell = detect_shell()?;

    let config_file = shell_config_path(&shell)?;
    let existing = fs::read_to_string(&config_file).unwrap_or_default();

    if existing.contains("q load") {
        eprintln!("Shell integration already installed in {}", config_file.display());
        return Ok(());
    }

    let line = match shell.as_str() {
        "fish" => "q load | source\n",
        _ => "eval \"$(q load)\"\n",
    };

    let mut content = existing;
    if !content.is_empty() && !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(line);

    if let Some(parent) = config_file.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create directory: {e}"))?;
    }

    fs::write(&config_file, content)
        .map_err(|e| format!("Failed to write {}: {e}", config_file.display()))?;

    eprintln!("Installed shell integration in {}", config_file.display());
    eprintln!("Restart your shell or run: source {}", config_file.display());
    Ok(())
}

fn detect_shell() -> Result<String, String> {
    let shell_path = env::var("SHELL").map_err(|_| "$SHELL is not set.")?;
    Ok(shell_path.rsplit('/').next().unwrap_or(&shell_path).to_string())
}

fn shell_config_path(shell: &str) -> Result<PathBuf, String> {
    let home = env::var("HOME").map(PathBuf::from)
        .map_err(|_| "$HOME is not set.")?;

    match shell {
        "bash" => Ok(home.join(".bashrc")),
        "zsh" => Ok(home.join(".zshrc")),
        "fish" => Ok(home.join(".config/fish/config.fish")),
        _ => Err(format!("Unsupported shell: {shell}. Supported: bash, zsh, fish.")),
    }
}

fn shell_snippet(shell: &str) -> String {
    match shell {
        "fish" => concat!(
            "function ?; q $argv; end\n",
            "function ??; q --yes $argv; end\n",
        ).to_string(),
        "zsh" => concat!(
            "alias '?'='noglob q'\n",
            "alias '??'='noglob q --yes'\n",
        ).to_string(),
        _ => concat!(
            "?() { q \"$@\"; }\n",
            "??() { q --yes \"$@\"; }\n",
        ).to_string(),
    }
}
