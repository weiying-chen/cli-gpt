use async_openai::{types::CreateCompletionRequestArgs, Client};
use futures::StreamExt;
use std::error::Error;
use tokio::io::{self, AsyncBufReadExt};
use tokio_stream::wrappers::LinesStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Setting up asynchronous read from stdin
    let stdin = io::stdin();
    let reader = io::BufReader::new(stdin);
    let mut lines = LinesStream::new(reader.lines());

    let mut prompt = String::new();

    // Reading multiple lines from stdin
    while let Some(line) = lines.next().await {
        let line = line?;
        prompt.push_str(&line);
        prompt.push('\n'); // Add newline character to separate lines
    }

    // Ensure that prompt is not empty
    if prompt.trim().is_empty() {
        return Err("No input provided".into());
    }

    // single request
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(&prompt)
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;

    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
