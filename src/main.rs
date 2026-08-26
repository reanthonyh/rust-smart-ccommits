mod ai_client;
mod diff_summarizer;
mod git_ops;
mod interactive_flow;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Parser, Debug)]
#[command(author, version, about = "AI-powered conventional commit generator")]
struct Args {
    /// Base URL for Ollama or llama.cpp (OpenAI compatible endpoint)
    #[arg(short, long, default_value = "http://localhost:11434/v1")]
    ai_url: String,

    /// Model name to use (e.g., "llama3", "mistral")
    #[arg(short, long, default_value = "llama3")]
    model: String,

    /// Force interactive mode (skip AI)
    #[arg(short, long)]
    interactive: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // 1. Get and summarize the diff
    let raw_diff = git_ops::get_staged_diff()?;
    if raw_diff.trim().is_empty() {
        println!("No staged changes found. Use `git add` first.");
        return Ok(());
    }

    let sanitized_diff = diff_summarizer::summarize(&raw_diff);

    // 2. Decide flow
    let commit_message = if args.interactive || !ai_client::is_ai_available(&args.ai_url).await {
        if !args.interactive {
            println!(
                "⚠️  AI server unreachable at {}. Falling back to interactive mode.",
                args.ai_url
            );
        }
        interactive_flow::run(&sanitized_diff)?
    } else {
        println!("🤖 Generating commit message with AI...");
        match ai_client::generate_message(&args.ai_url, &args.model, &sanitized_diff).await {
            Ok(msg) => {
                println!(
                    "\n--- AI Generated Message ---\n{}\n----------------------------\n",
                    msg
                );
                interactive_flow::confirm_or_edit(&msg)?
            }
            Err(e) => {
                println!(
                    "⚠️  AI generation failed: {}. Falling back to interactive mode.",
                    e
                );
                interactive_flow::run(&sanitized_diff)?
            }
        }
    };

    // 3. Commit
    git_ops::commit(&commit_message)?;
    println!("✅ Successfully committed!");

    Ok(())
}
