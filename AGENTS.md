# smart_commit AGENTS.md

## Commands

- `cargo run` — start the tool. Args: `--ai-url`, `--model`, `--interactive`
- `cargo test` — run tests (there are currently no test files)
- `cargo build` — compile the project

## How it works

1. `git diff --staged` gets the staged diff
2. `diff_summarizer::summarize()` filters out lockfiles/generated files and truncates long lines
3. If `--interactive` or AI is unreachable → `interactive_flow::run()` prompts for type, emoji, title, body
4. Otherwise `ai_client::generate_message()` calls Ollama/OpenAI at `{ai_url}/chat/completions` with a strict system prompt
5. The generated or confirmed message is passed to `git commit -m "<message>"`

## AI format

Output must be `<type>: <gitmoji> <Title>` with extra text. Types: feat, fix, docs, style, refactor, perf, test, build, ci, chore. Title max 50 chars. Body wrapped at 72 chars.

## Gotchas

- No AI server = falls back to interactive mode (warns at `ai_client.rs:41-44`)
- AI may wrap output in ``` — the code strips ``` at `ai_client.rs:86`
- Lockfiles and `dist/`/`build/` are auto-skipped in diff summarization (`diff_summarizer.rs:10-19`)
- Interactive mode uses `inquire` crate for prompts — no scriptable input

## Development

- `AI_URL` and `MODEL` env vars are not required; defaults are `http://localhost:11434/v1` and `llama3`
- If `Ollama` is running locally, the tool works out-of-the-box
- To test interactive flow without AI: `cargo run -- --interactive`