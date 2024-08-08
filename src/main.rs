pub mod groq {
    use reqwest::Client;
    use serde::Serialize;
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
            let request_body = CreateChatCompletionRequest {
                model: "mixtral-8x7b-32768".to_string(),
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

            let text = response.text().await?;
            Ok(text)
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
}

use futures::StreamExt;
use groq::Groq;
use std::{env, error::Error};
use tokio::io::{self, AsyncBufReadExt};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = read_input().await?;

    if prompt.trim().is_empty() {
        return Err("No input provided".into());
    }

    let api_key = env::var("GROQ_API_KEY").expect("GROQ_API_KEY environment variable not set");
    let groq_client = Groq::new(api_key);
    let result = groq_client.send_request(&prompt).await;

    match result {
        Ok(response_text) => println!("{}", response_text),
        Err(e) => eprintln!("Request failed: {}", e),
    }

    Ok(())
}

async fn read_input() -> Result<String, io::Error> {
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut lines = LinesStream::new(reader.lines());

    let mut prompt = String::new();

    while let Some(line) = lines.next().await {
        let line = line?;
        prompt.push_str(&line);
        prompt.push('\n');
    }

    Ok(prompt)
}
