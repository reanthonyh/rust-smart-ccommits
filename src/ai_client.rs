use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::debug;

#[derive(Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
    max_tokens: u32,
}

#[derive(Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: Message,
}

pub async fn is_ai_available(base_url: &str) -> bool {
    // A simple ping to the models endpoint or just a lightweight chat request
    let client = Client::new();
    let url = format!("{}/models", base_url);
    client.get(&url).send().await.is_ok()
}

pub async fn generate_message(base_url: &str, model: &str, diff: &str) -> Result<String> {
    let client = Client::new();
    let url = format!("{}/chat/completions", base_url);

    let system_prompt = r#"You are an expert developer writing Git commit messages.
You must strictly follow this exact format without any markdown formatting or extra text:
<type>: <gitmoji> <Title>

<Extra Text>

Rules:
- <type> must be one of: feat, fix, docs, style, refactor, perf, test, build, ci, chore.
- <gitmoji> must be a relevant emoji (e.g., ✨ for feat, 🐛 for fix, 📝 for docs).
- <Title> should be a concise summary (imperative mood, max 50 chars).
- <Extra Text> should explain the 'why' and 'what', wrapped at 72 chars.
- DO NOT wrap the output in ``` or add introductory text."#;

    let request = ChatRequest {
        model: model.to_string(),
        messages: vec![
            Message {
                role: "system".into(),
                content: system_prompt.into(),
            },
            Message {
                role: "user".into(),
                content: format!("Generate a commit message for this diff:\n\n{}", diff),
            },
        ],
        temperature: 0.3, // Low temperature for strict formatting
        max_tokens: 500,
    };

    let response = client
        .post(&url)
        .json(&request)
        .send()
        .await
        .context("Failed to send request to AI server")?;

    let status = response.status();
    let response_text = response.text().await.context("Failed to read response body")?;
    
    debug!("AI response status: {}", status);
    debug!("AI raw response: {}", response_text);

    let chat_response: ChatResponse = serde_json::from_str(&response_text)
        .context("Failed to parse AI response")?;

    let content = chat_response
        .choices
        .first()
        .context("AI returned empty choices")?
        .message
        .content
        .clone();

    // Clean up any accidental markdown blocks the LLM might have added
    let cleaned = content.replace("```", "").trim().to_string();

    Ok(cleaned)
}
