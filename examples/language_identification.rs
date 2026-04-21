use sarvam::{types::LanguageIdentificationRequest, SarvamClient};

#[tokio::main]
async fn main() {
    let api_key = std::env::var("SARVAM_API_KEY").expect("SARVAM_API_KEY not set");
    let client = SarvamClient::new(&api_key);

    let request = LanguageIdentificationRequest {
        input: "यह एक उदाहरण वाक्य है जो हिंदी भाषा में लिखा गया है।".to_string(),
    };

    let response = client.text().identify_language(request).await.unwrap();
    println!("Language: {:?}", response.language_code);
    println!("Script: {:?}", response.script_code);
}
