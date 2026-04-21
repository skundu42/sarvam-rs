use base64::Engine;
use sarvam::{streaming::TtsMessage, types::*, SarvamClient};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let mut stream = client
        .text_to_speech()
        .stream()
        .model(TextToSpeechModel::BulbulV3)
        .target_language_code(TextToSpeechLanguage::HiIn)
        .speaker(TextToSpeechSpeaker::Shubh)
        .connect()
        .await
        .unwrap();

    stream
        .send_text("नमस्ते, यह एक स्ट्रीमिंग टेस्ट है।")
        .await
        .unwrap();
    stream.flush().await.unwrap();

    let mut audio_chunks = Vec::new();
    while let Some(result) = stream.next().await {
        match result {
            Ok(TtsMessage::Audio(audio)) => {
                let audio_bytes = base64::engine::general_purpose::STANDARD
                    .decode(&audio.data.audio)
                    .expect("Failed to decode base64 audio chunk");
                println!("Received audio chunk ({} bytes)", audio_bytes.len());
                audio_chunks.push(audio.data.audio);
            }
            Ok(TtsMessage::Event(event)) => {
                println!("Event: {:?}", event.data.event_type);
                break;
            }
            Ok(TtsMessage::Error(error)) => {
                eprintln!("Error: {}", error.data.message);
                break;
            }
            Err(e) => {
                eprintln!("Stream error: {}", e);
                break;
            }
        }
    }

    println!("Received {} audio chunks", audio_chunks.len());
}
