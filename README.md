# openai-rs

A Rust async library for interacting with the OpenAI API.

## Features

- Async client using `tokio` and `reqwest`
- Typed request and response models for OpenAI Chat Completions
- Error handling with `thiserror`
- Example usage included

## Usage

Set your OpenAI API key in the environment:

```sh
export OPENAI_API_KEY=sk-...
```

Run the example:

```sh
cargo run --example chat_completion
```

Example code:

```rust
use openai_rs::{OpenAIClient, ChatCompletionRequest, ChatMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("OPENAI_API_KEY")?;
    let client = OpenAIClient::new(api_key);

    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: "Hello, who are you?".to_string(),
    }];

    let request = ChatCompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages,
        max_tokens: Some(16),
        temperature: Some(0.7),
        top_p: None,
        n: None,
        stream: None,
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        user: None,
    };

    let response = client.create_chat_completion(request).await?;
    println!("{:?}", response);

    Ok(())
}
```

## License

Licensed under [Apache License, Version 2.0](LICENSE-APACHE).
