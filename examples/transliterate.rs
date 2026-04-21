use sarvam::{types::*, SarvamClient};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = TransliterationRequest {
        input: "मैं ऑफिस जा रहा हूँ".to_string(),
        source_language_code: TransliterateSourceLanguage::HiIn,
        target_language_code: TransliterateTargetLanguage::EnIn,
        numerals_format: None,
        spoken_form_numerals_language: None,
        spoken_form: None,
    };

    let response = client.text().transliterate(request).await.unwrap();
    println!("Transliterated: {}", response.transliterated_text);
}
