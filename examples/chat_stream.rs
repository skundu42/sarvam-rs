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
            content: "Tell me a story about a brave knight.".to_string(),
        }],
        model: ChatModel::Sarvam105b,
        temperature: Some(0.7),
        stream: Some(true),
        ..Default::default()
    };

    let mut stream = client.chat().completions_stream(request).await.unwrap();

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                for choice in &chunk.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    println!();
}
