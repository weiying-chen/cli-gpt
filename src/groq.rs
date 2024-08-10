use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

pub struct Groq {
    api_key: String,
    client: Client,
}

impl Groq {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn send_request(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        // Capture the original indentation level
        let original_indentation = get_original_indentation(prompt);

        let request_body = CreateChatCompletionRequest {
            model: "llama-3.1-8b-instant".to_string(),
            messages: vec![Message {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        let response_body: CreateChatCompletionResponse = response.json().await?;
        let content = response_body
            .choices
            .first()
            .ok_or("No choices found in response")?
            .message
            .content
            .clone();

        let extracted_code = extract_code_block(&content);
        let reindented_code = apply_indentation(&extracted_code, original_indentation);

        Ok(reindented_code)
    }
}

#[derive(Serialize)]
struct CreateChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
}

#[derive(Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct CreateChatCompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: MessageContent,
}

#[derive(Deserialize)]
struct MessageContent {
    content: String,
}

fn extract_code_block(content: &str) -> String {
    let start_tag = "```";
    let end_tag = "```";

    if let Some(start_idx) = content.find(start_tag) {
        let remaining_content = &content[start_idx + start_tag.len()..];
        let remaining_content_trimmed = remaining_content.trim_start();

        // Check if there's a language tag (e.g., "javascript")
        let first_newline = remaining_content_trimmed.find('\n').unwrap_or(0);
        let code_start_idx = if first_newline > 0
            && remaining_content_trimmed[..first_newline].contains(|c: char| !c.is_whitespace())
        {
            start_idx + start_tag.len() + first_newline + 1
        } else {
            start_idx + start_tag.len()
        };

        if let Some(end_idx) = content[code_start_idx..].find(end_tag) {
            let start = code_start_idx;
            let end = code_start_idx + end_idx;
            return content[start..end].to_string();
        }
    }

    "Code block not found".to_string()
}

fn get_original_indentation(prompt: &str) -> usize {
    prompt
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.chars().take_while(|c| c.is_whitespace()).count())
        .min()
        .unwrap_or(0)
}

fn apply_indentation(code: &str, indentation: usize) -> String {
    let indent_str = " ".repeat(indentation);
    code.lines()
        .map(|line| format!("{}{}", indent_str, line))
        .collect::<Vec<String>>()
        .join("\n")
}
