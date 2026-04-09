mod config;
mod llm;
mod shell;
mod spinner;

use std::io::{self, BufRead, Write};
use std::process;

fn print_usage() {
    eprintln!(
        "Usage: q [OPTIONS] <query>\n\
         \n\
         Translate natural language into shell commands.\n\
         \n\
         Options:\n\
         \x20 -y, --yes    Run the command without confirmation\n\
         \x20 -h, --help   Print this help message\n\
         \n\
         Subcommands:\n\
         \x20 init          Install shell integration (? and ?? functions)\n\
         \x20 load          Print shell integration snippet to stdout"
    );
}

fn run() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        print_usage();
        process::exit(1);
    }

    if args[0] == "--help" || args[0] == "-h" {
        print_usage();
        return Ok(());
    }

    if args[0] == "init" {
        return shell::init();
    }

    if args[0] == "load" {
        return shell::load();
    }

    let mut auto_run = false;
    let query_args = if args[0] == "--yes" || args[0] == "-y" {
        auto_run = true;
        &args[1..]
    } else {
        &args[..]
    };

    let query = query_args.join(" ");
    if query.is_empty() {
        print_usage();
        process::exit(1);
    }

    let config = config::Config::from_env()?;
    let command = spinner::with_spinner(|| llm::call(&config, &query))?;
    let command = command.trim();

    // Print with cyan ❯ prefix
    println!("\x1b[36m❯\x1b[0m {command}");

    if !auto_run {
        eprint!("Run? [y/N] ");
        io::stderr().flush().ok();

        let mut input = String::new();
        io::stdin()
            .lock()
            .read_line(&mut input)
            .map_err(|e| format!("Failed to read input: {e}"))?;

        if input.trim().to_lowercase() != "y" {
            eprintln!("Aborted.");
            return Ok(());
        }
    }

    let status = process::Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .map_err(|e| format!("Failed to execute command: {e}"))?;

    process::exit(status.code().unwrap_or(1));
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
