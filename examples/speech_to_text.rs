use sarvam::{types::*, SarvamClient};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let file_path = std::env::var("AUDIO_FILE").unwrap_or_else(|_| "audio.wav".to_string());

    let response = client
        .speech_to_text()
        .transcribe(&file_path)
        .model(SpeechToTextModel::SaarasV3)
        .mode(SttMode::Transcribe)
        .language_enum(SpeechToTextLanguage::Unknown)
        .send()
        .await
        .unwrap();

    println!("Transcript: {}", response.transcript);
    println!("Language: {:?}", response.language_code);
}
