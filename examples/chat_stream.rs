use sarvam::{
    types::{ChatCompletionRequest, ChatMessage, ChatModel, ReasoningEffort},
    SarvamClient,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = ChatCompletionRequest {
        messages: vec![ChatMessage::User {
            content: "Explain Rust ownership in 3 short bullet points.".to_string(),
        }],
        model: ChatModel::Sarvam30b,
        temperature: Some(0.2),
        reasoning_effort: Some(ReasoningEffort::Low),
        stream: Some(true),
        max_tokens: Some(256),
        ..Default::default()
    };

    let mut stream = client.chat().completions_stream(request).await.unwrap();
    let mut printed_anything = false;

    while let Some(result) = stream.next().await {
        match result {
            Ok(chunk) => {
                for choice in &chunk.choices {
                    if let Some(content) = &choice.delta.content {
                        print!("{}", content);
                        printed_anything = true;
                    } else if let Some(reasoning) = &choice.delta.reasoning_content {
                        print!("{}", reasoning);
                        printed_anything = true;
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if printed_anything {
        println!();
    } else {
        println!("No visible streamed content returned.");
    }
}
