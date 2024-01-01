use async_openai::{types::CreateCompletionRequestArgs, Client};
use std::env;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure there is at least one argument (excluding the program name)
    if args.len() < 2 {
        return Err("Usage: program <prompt>".into());
    }

    // Use the first argument as the prompt
    let prompt = &args[1];

    // single request
    let request = CreateCompletionRequestArgs::default()
        .model("text-davinci-003")
        .prompt(prompt)
        .max_tokens(40_u16)
        .build()?;

    let response = client.completions().create(request).await?;

    for choice in response.choices {
        println!("{}", choice.text);
    }

    Ok(())
}
