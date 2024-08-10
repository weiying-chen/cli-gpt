mod groq;

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
