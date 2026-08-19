use anyhow::{Context, Result};
use std::process::Command;

pub fn get_staged_diff() -> Result<String> {
    let output = Command::new("git")
        .args(["diff", "--staged"])
        .output()
        .context("Failed to execute git diff. Is this a git repository?")?;

    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub fn commit(message: &str) -> Result<()> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to execute git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Git commit failed: {}", stderr);
    }
    Ok(())
}
