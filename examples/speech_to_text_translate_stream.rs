use sarvam::{
    streaming::{SttAudioEncoding, SttMessage, SttStreamingTranslateModel, WebSocketSampleRate},
    types::SttMode,
    SarvamClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let audio_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "audio.wav".to_string());
    let audio = std::fs::read(audio_path)?;

    let client = SarvamClient::new(api_key);
    let mut stream = client
        .speech_to_text_translate()
        .stream()
        .model(SttStreamingTranslateModel::SaarasV3)
        .mode(SttMode::Translate)
        .flush_signal(true)
        .connect()
        .await?;

    stream
        .send_audio(
            audio,
            SttAudioEncoding::AudioWav,
            WebSocketSampleRate::Hz16000,
        )
        .await?;
    stream.flush().await?;

    while let Some(message) = stream.next().await {
        match message? {
            SttMessage::Transcript(transcript) => {
                if let Some(text) = transcript.text() {
                    println!("Translated: {text}");
                    break;
                }
            }
            SttMessage::Event(event) => println!("Event: {}", event.event_type),
            SttMessage::Error(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
            SttMessage::Raw(raw) => println!("Raw: {}", raw),
        }
    }

    Ok(())
}
