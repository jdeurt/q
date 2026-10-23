# q

Translate natural language into shell commands.

```
? list all files larger than 10mb
❯ find . -size +10M -ls
Run? [y/N]
```

## Installation

```sh
cargo install --git https://github.com/jdeurt/q
```

## Setup

By default, Q uses Anthropic with Claude Haiku 5.5. Set your API key:

```sh
export Q_ANTHROPIC_API_KEY="your-key-here"
```

To use OpenAI with GPT-6 Luna:

```sh
export Q_PROVIDER="openai"
export Q_OPENAI_API_KEY="your-key-here"
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
| `Q_PROVIDER` | Provider: `anthropic` (default) or `openai` |
| `Q_ANTHROPIC_API_KEY` | Anthropic API key (required when using Anthropic) |
| `Q_OPENAI_API_KEY` | OpenAI API key (required when using OpenAI) |
| `Q_MODEL` | Model override for the selected provider (default: `claude-haiku-5-5` for Anthropic, `gpt-6-luna` for OpenAI) |
| `Q_EFFORT` | Optional effort level. Unset or blank omits effort from the request, leaving the provider's default in effect. |

Set effort for an individual query or export it for every query:

```sh
Q_EFFORT=low q list all files larger than 10mb
export Q_EFFORT="high"
```

Anthropic accepts `low`, `medium`, `high`, `xhigh`, and `max`. OpenAI accepts `none`,
`minimal`, `low`, `medium`, `high`, `xhigh`, and `max`. Support varies by model;
choose a level supported by your `Q_MODEL`. The default OpenAI model, GPT-6 Luna,
supports all listed OpenAI levels except `minimal`. `Q_EFFORT=none` explicitly
disables OpenAI reasoning on models that support it; unset `Q_EFFORT` to use the
provider's default.

See the [Anthropic effort guide](https://platform.claude.com/docs/en/build-with-claude/effort)
and [OpenAI model documentation](https://developers.openai.com/api/docs/models/gpt-6-luna)
for model-specific support.
