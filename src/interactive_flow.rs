use anyhow::Result;
use inquire::{Confirm, Select, Text};
use inquire::validator::Validation;

const TYPES: &[&str] = &[
    "feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore",
];

const GITMOJIS: &[&str] = &[
    "✨ (feat)",
    "🐛 (fix)",
    "📝 (docs)",
    "💄 (style)",
    "♻️ (refactor)",
    "⚡️ (perf)",
    "✅ (test)",
    "👷 (build)",
    "💚 (ci)",
    "🔧 (chore)",
];

pub fn run(sanitized_diff: &str) -> Result<String> {
    println!("\n--- Staged Changes Summary ---");
    // Print first 20 lines of diff so user has context
    let diff_preview: String = sanitized_diff
        .lines()
        .take(20)
        .collect::<Vec<&str>>()
        .join("\n");
    println!("{}\n... (truncated for display)\n", diff_preview);

    let commit_type = Select::new("Select commit type:", TYPES.to_vec()).prompt()?;

    let emoji_selection = Select::new("Select gitmoji:", GITMOJIS.to_vec()).prompt()?;
    let emoji = emoji_selection.split_whitespace().next().unwrap_or("✨");

    let title = Text::new("Commit title (imperative mood, e.g., 'add user login'):")
        .with_validator(|val: &str| {
            if val.is_empty() {
                Err("Title cannot be empty".into())
            } else if val.len() > 50 {
                Err("Keep it under 50 characters".into())
            } else {
                Ok(Validation::Valid)
            }
        })
        .prompt()?;

    let body = Text::new("Extra text (optional, press Enter to skip):")
        .prompt()
        .unwrap_or_default();

    let final_message = format!("{}: {} {}\n\n{}", commit_type, emoji, title, body);

    confirm_or_edit(&final_message)
}

pub fn confirm_or_edit(message: &str) -> Result<String> {
    let mut current_msg = message.to_string();

    loop {
        println!(
            "\n--- Final Commit Message ---\n{}\n----------------------------",
            current_msg
        );

        let confirm = Confirm::new("Commit with this message?")
            .with_default(true)
            .prompt()?;

        if confirm {
            return Ok(current_msg);
        } else {
            let edit_choice = Select::new(
                "What would you like to do?",
                vec!["Edit the message manually", "Start over"],
            )
            .prompt()?;

            if edit_choice == "Edit the message manually" {
                current_msg = Text::new("Edit message:")
                    .with_default(&current_msg)
                    .prompt()?;
            } else {
                // If they want to start over, we just return the current one,
                // but in a real app you might loop back to the `run` function.
                return Ok(current_msg);
            }
        }
    }
}
