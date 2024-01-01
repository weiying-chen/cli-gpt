use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // single request
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt("Tell me a joke about the universe")
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;

    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
