use sarvam::{types::*, SarvamClient};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = TranslationRequest {
        input: "मैं ऑफिस जा रहा हूँ".to_string(),
        source_language_code: TranslateSourceLanguage::HiIn,
        target_language_code: TranslateTargetLanguage::EnIn,
        speaker_gender: None,
        mode: None,
        model: None,
        output_script: None,
        numerals_format: None,
    };

    let response = client.text().translate(request).await.unwrap();
    println!("Translated: {}", response.translated_text);
    println!("Source language: {}", response.source_language_code);
}
