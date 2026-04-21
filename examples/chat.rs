use sarvam::{
    types::{ChatCompletionRequest, ChatMessage, ChatModel, FinishReason, ReasoningEffort},
    SarvamClient,
};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = ChatCompletionRequest {
        messages: vec![ChatMessage::User {
            content: "Explain the theory of relativity in 3 short bullet points.".to_string(),
        }],
        model: ChatModel::Sarvam30b,
        temperature: Some(0.2),
        reasoning_effort: Some(ReasoningEffort::Low),
        max_tokens: Some(256),
        ..Default::default()
    };

    let response = client.chat().completions(request).await.unwrap();
    for choice in &response.choices {
        println!("Finish reason: {:?}", choice.finish_reason);

        if let Some(content) = &choice.message.content {
            println!("Assistant: {}", content);
        } else if let Some(reasoning) = &choice.message.reasoning_content {
            println!("Reasoning: {}", reasoning);
        } else {
            println!("No visible assistant content returned.");
        }

        if matches!(choice.finish_reason, FinishReason::Length) {
            println!(
                "The response hit the max token limit before finishing. Increase max_tokens if needed."
            );
        }
    }
    println!("Usage: {:?}", response.usage);
}
