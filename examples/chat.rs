use sarvam::{
    types::{ChatCompletionRequest, ChatMessage, ChatModel},
    SarvamClient,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = ChatCompletionRequest {
        messages: vec![ChatMessage::User {
            content: "Hello, can you explain the theory of relativity?".to_string(),
        }],
        model: ChatModel::Sarvam105b,
        temperature: Some(0.7),
        max_tokens: Some(1024),
        ..Default::default()
    };

    let response = client.chat().completions(request).await.unwrap();
    for choice in &response.choices {
        if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
        }
    }
    println!("Usage: {:?}", response.usage);
}
