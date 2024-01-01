use async_openai::types::ChatCompletionRequestUserMessageArgs;
use async_openai::{types::CreateChatCompletionRequestArgs, Client};
use futures::StreamExt;
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let prompt = read_input().await?;

    if prompt.trim().is_empty() {
        return Err("No input provided".into());
    }

    let response = send_request(&prompt).await?;

    for choice in response.choices {
        if let Some(content) = choice.message.content {
            println!("{}", content);
        }
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

async fn send_request(
    prompt: &str,
) -> Result<async_openai::types::CreateChatCompletionResponse, Box<dyn Error>> {
    let client = Client::new();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-3.5-turbo")
        .max_tokens(512u16)
        .messages([ChatCompletionRequestUserMessageArgs::default()
            .content(prompt)
            .build()?
            .into()])
        .build()?;

    let response = client.chat().create(request).await?;

    Ok(response)
}
