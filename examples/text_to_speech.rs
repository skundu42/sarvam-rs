use sarvam::{
    types::*,
    SarvamClient,
};
use base64::Engine;

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = TextToSpeechRequest {
        text: "Hello, welcome to Sarvam's text-to-speech service.".to_string(),
        target_language_code: TextToSpeechLanguage::EnIn,
        speaker: Some(TextToSpeechSpeaker::Shubh),
        model: Some(TextToSpeechModel::BulbulV3),
        pace: Some(1.0),
        ..Default::default()
    };

    let response = client.text_to_speech().convert(request).await.unwrap();

    for (i, audio_b64) in response.audios.iter().enumerate() {
        let audio_bytes = base64::engine::general_purpose::STANDARD
            .decode(audio_b64)
            .expect("Failed to decode base64 audio");
        let out_path = format!("output_{}.wav", i);
        std::fs::write(&out_path, &audio_bytes).expect("Failed to write audio file");
        println!("Saved audio to {}", out_path);
    }
}
