# sarvam-rs

An unofficial Rust SDK for [Sarvam AI](https://sarvam.ai) APIs — chat completions, translation, speech-to-text, text-to-speech, transliteration, and language identification.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
sarvam = { package = "sarvam-rs", version = "0.1" }
```

Streaming support (SSE-based chat streaming and WebSocket-based TTS streaming) is enabled by default. To disable it:

```toml
[dependencies]
sarvam = { package = "sarvam-rs", version = "0.1", default-features = false }
```

## Quick Start

```rust
use sarvam::{SarvamClient, types::chat::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SarvamClient::new("your-api-key");

    let request = ChatCompletionRequest {
        messages: vec![ChatMessage::User {
            content: "Hello, how are you?".to_string(),
        }],
        model: ChatModel::SarvamM,
        temperature: Some(0.7),
        max_tokens: Some(256),
        ..Default::default()
    };

    let response = client.chat().completions(request).await?;

    for choice in &response.choices {
        println!("{}", choice.message.content.as_deref().unwrap_or(""));
    }

    Ok(())
}
```

## API Reference

### Client Configuration

```rust
use std::time::Duration;
use sarvam::{SarvamClient, SarvamConfig};

// Simple
let client = SarvamClient::new("your-api-key");

// With custom config
let client = SarvamClient::from_config(
    SarvamConfig::new("your-api-key")
        .base_url("https://api.sarvam.ai")
        .timeout(Duration::from_secs(120)),
);
```

### Chat Completions

```rust
use sarvam::types::chat::*;

let request = ChatCompletionRequest {
    messages: vec![
        ChatMessage::System {
            content: "You are a helpful assistant.".to_string(),
        },
        ChatMessage::User {
            content: "Explain Rust in one sentence.".to_string(),
        },
    ],
    model: ChatModel::SarvamM,
    temperature: Some(0.7),
    max_tokens: Some(1024),
    ..Default::default()
};

let response = client.chat().completions(request).await?;
```

Available models: `Sarvam105b`, `Sarvam30b`, `SarvamM`.

#### Streaming Chat

```rust
let mut stream = client.chat().completions_stream(request).await?;

while let Some(result) = stream.next().await {
    let chunk = result?;
    if let Some(content) = chunk.choices[0].delta.content.as_deref() {
        print!("{content}");
    }
}
```

### Translation

```rust
use sarvam::types::translate::*;

let request = TranslationRequest {
    input: "नमस्ते दुनिया".to_string(),
    source_language_code: TranslateSourceLanguage::HiIn,
    target_language_code: TranslateTargetLanguage::EnIn,
    speaker_gender: None,
    mode: None,
    model: None,
    output_script: None,
    numerals_format: None,
};

let response = client.text().translate(request).await?;
println!("{}", response.translated_text);
```

### Transliteration

```rust
use sarvam::types::transliterate::*;

let request = TransliterationRequest {
    input: "namaste".to_string(),
    source_language_code: TransliterateSourceLanguage::EnIn,
    target_language_code: TransliterateTargetLanguage::HiIn,
    numerals_format: None,
    spoken_form_numerals_language: None,
    spoken_form: None,
};

let response = client.text().transliterate(request).await?;
println!("{}", response.transliterated_text);
```

### Language Identification

```rust
use sarvam::types::transliterate::*;

let request = LanguageIdentificationRequest {
    input: "यह हिंदी में लिखा हुआ है".to_string(),
};

let response = client.text().identify_language(request).await?;
println!("{} ({})", response.language_code.unwrap_or_default(), response.script_code.unwrap_or_default());
```

### Speech-to-Text

```rust
use sarvam::types::speech_to_text::*;

let response = client
    .speech_to_text()
    .transcribe("audio.wav")
    .model(SpeechToTextModel::SaarikaV2_5)
    .language_enum(SpeechToTextLanguage::HiIn)
    .send()
    .await?;

println!("{}", response.transcript);
```

### Speech-to-Text Translate

Transcribe audio and translate to English in one step:

```rust
let response = client
    .speech_to_text_translate()
    .translate("audio.wav")
    .send()
    .await?;

println!("{}", response.transcript);
```

### Text-to-Speech

```rust
use sarvam::types::text_to_speech::*;
use base64::Engine;

let request = TextToSpeechRequest {
    text: "Hello, welcome to Sarvam AI.".to_string(),
    target_language_code: TextToSpeechLanguage::EnIn,
    speaker: Some(TextToSpeechSpeaker::Shubh),
    model: Some(TextToSpeechModel::BulbulV3),
    ..Default::default()
};

let response = client.text_to_speech().convert(request).await?;
let audio_bytes = base64::engine::general_purpose::STANDARD.decode(&response.audios[0])?;
std::fs::write("output.wav", audio_bytes)?;
```

#### Streaming TTS (WebSocket)

```rust
use base64::Engine;

let mut stream = client
    .text_to_speech()
    .stream()
    .model(TextToSpeechModel::BulbulV3)
    .target_language_code(TextToSpeechLanguage::EnIn)
    .speaker(TextToSpeechSpeaker::Shubh)
    .connect()
    .await?;

stream.send_text("नमस्ते दुनिया").await?;
stream.flush().await?;

while let Some(result) = stream.next().await {
    match result? {
        sarvam::streaming::TtsMessage::Audio(audio) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(&audio.data.audio)?;
            // process audio bytes
        }
        sarvam::streaming::TtsMessage::Event(event) => {
            println!("{:?}", event);
        }
        sarvam::streaming::TtsMessage::Error(err) => {
            eprintln!("{}", err.data.message);
        }
    }
}
```

## Error Handling

All API methods return `Result<T, SarvamError>`:

```rust
use sarvam::SarvamError;

match client.chat().completions(request).await {
    Ok(response) => { /* ... */ }
    Err(SarvamError::ApiError { code, message, .. }) => {
        eprintln!("API error ({code}): {message}");
    }
    Err(e) => return Err(e.into()),
}
```

## Supported Languages

The SDK supports 22+ Indic languages across all APIs including Hindi, Bengali, Kannada, Malayalam, Marathi, Odia, Punjabi, Tamil, Telugu, Gujarati, Urdu, Nepali, and more. Language codes follow the `xx-IN` format (e.g., `hi-IN`, `ta-IN`).

## Feature Flags

| Feature   | Default | Description                                       |
|-----------|---------|---------------------------------------------------|
| `streaming` | Yes   | SSE-based chat streaming and WebSocket-based TTS  |

## Examples

See the [`examples/`](./examples) directory for complete working examples:

| Example | Description |
|---------|-------------|
| `chat` | Basic chat completion |
| `chat_stream` | Streaming chat completion |
| `translate` | Text translation |
| `transliterate` | Text transliteration |
| `language_identification` | Language detection |
| `speech_to_text` | Audio transcription |
| `speech_to_text_translate` | Audio transcription + translation |
| `text_to_speech` | Text-to-audio conversion |
| `text_to_speech_stream` | Streaming text-to-audio via WebSocket |

Run an example:

```bash
SARVAM_API_KEY=your-key cargo run --example chat
```

## Release Automation

This repo includes GitHub Actions workflows for CI and crates.io publishing:

- `.github/workflows/ci.yml` runs `cargo test` and `cargo publish --dry-run` on pushes to `main` and on pull requests.
- `.github/workflows/publish.yml` publishes to crates.io whenever a GitHub Release is published.

Before using the publish workflow:

1. Create a crates.io API token at [crates.io/me](https://crates.io/me).
2. Add it to the GitHub repository as the `CARGO_REGISTRY_TOKEN` Actions secret.
3. Bump the `version` in `Cargo.toml`.
4. Create a GitHub Release with a tag that matches the crate version, such as `0.2.0` or `v0.2.0`.

The publish workflow validates that the release tag matches `Cargo.toml`, reruns tests, performs `cargo publish --dry-run`, and then publishes the crate.

## License

MIT
