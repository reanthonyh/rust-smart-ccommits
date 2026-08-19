# smart_commit

AI-powered conventional commit message generator for Rust projects.

## Description

`smart_commit` is a CLI tool that generates conventional commit messages using AI (Ollama or OpenAI-compatible endpoints). It works in two modes:

1. **AI mode**: Calls an LLM to generate a commit message from your staged diff
2. **Interactive mode**: Prompts you for commit type, emoji, title, and optional body

The generated message follows the `<type>: <gitmoji> <Title>` format and is passed directly to `git commit`.

## How to Use

### Basic usage

```bash
# Ensure Ollama is running locally (default: llama3 model)
cargo run

# Or run the compiled binary
./target/debug/smart_commit
```

### CLI arguments

| Arg             | Short | Description                                    | Default                     |
| --------------- | ----- | ---------------------------------------------- | --------------------------- |
| `--ai-url`      | `-u`  | Base URL for Ollama/OpenAI-compatible endpoint | `http://localhost:11434/v1` |
| `--model`       | `-m`  | Model name to use (e.g., `llama3`, `mistral`)  | `llama3`                    |
| `--interactive` | `-i`  | Force interactive mode (skip AI)               | —                           |

### Workflow

1. Stage your changes: `git add <files>`
2. Run `smart_commit` — it will:
   - Get the staged diff
   - If AI server is reachable, generate a commit message
   - Otherwise fall back to interactive prompts
   - Prompt to confirm/edit the message
   - Commit with `git commit -m "<message>"`

### Without AI server

```bash
cargo run -- --interactive
```

This skips the AI check and directly prompts for type, emoji, title, and body.

### AI format

Output follows `<type>: <gitmoji> <Title>` where:

- **Type**: one of `feat`, `fix`, `docs`, `style`, `refactor`, `perf`, `test`, `build`, `ci`, `chore`
- **Gitmoji**: relevant emoji (e.g., ✨ for feat, 🐛 for fix)
- **Title**: concise summary in imperative mood, max 50 chars
- **Extra text**: explains why/what, wrapped at 72 chars

## Development

- `cargo build` — compile
- `cargo test` — run tests (none currently written)
- `AI_URL` and `MODEL` env vars are optional; defaults are `http://localhost:11434/v1` and `llama3`
- Lockfiles and `dist/`/`build/` directories are automatically skipped in diff summarization
