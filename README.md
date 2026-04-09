# q

Translate natural language into shell commands.

```
? list all files larger than 10mb
❯ find . -size +10M -ls
Run? [y/N]
```

## Installation

```sh
cargo install --git https://github.com/juandeurtubey/q
```

## Setup

Set your API key:

```sh
export Q_ANTHROPIC_API_KEY="your-key-here"
```

Install shell integration:

```sh
q init
```

This adds `?` and `??` functions to your shell. Restart your shell or source your config to activate.

## Usage

```sh
? <query>          # generate a command, confirm before running
?? <query>         # generate and run immediately
```

You can also call the binary directly:

```sh
q <query>
q --yes <query>
```

## Configuration

| Variable | Description |
|---|---|
| `Q_ANTHROPIC_API_KEY` | API key (required) |
| `Q_MODEL` | Model override (default: `claude-haiku-4-5`) |
