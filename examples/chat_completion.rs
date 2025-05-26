use openai_rs::{ChatCompletionRequest, ChatMessage, OpenAIClient};

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
