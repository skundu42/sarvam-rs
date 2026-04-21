# sarvam-rs

An unofficial Rust SDK for [Sarvam AI](https://sarvam.ai) APIs — chat completions, translation, transliteration, language identification, speech-to-text, speech-to-text batch jobs, speech-to-text WebSocket streaming, text-to-speech, and Document Intelligence job workflows.

## Installation


```
cargo add sarvam-rs
```

or

Add to your `Cargo.toml`:

```toml
[dependencies]
sarvam = { package = "sarvam-rs", version = "0.2.0" }
```

Package name on crates.io: `sarvam-rs`
Rust import path in code: `sarvam`

Streaming support (SSE-based chat streaming, STT/STTT WebSocket streaming, and WebSocket-based TTS streaming) is enabled by default. To disable it:

```toml
[dependencies]
sarvam = { package = "sarvam-rs", version = "0.2.0", default-features = false }
```

## Quick Start

```rust
use sarvam::{SarvamClient, types::chat::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = SarvamClient::new("your-api-key");

    let request = ChatCompletionRequest {
        messages: vec![ChatMessage::User {
            content: "Explain Rust ownership in 3 short bullet points.".to_string(),
        }],
        model: ChatModel::Sarvam30b,
        temperature: Some(0.2),
        reasoning_effort: Some(ReasoningEffort::Low),
        max_tokens: Some(256),
        ..Default::default()
    };

    let response = client.chat().completions(request).await?;

    for choice in &response.choices {
        if let Some(content) = choice.message.content.as_deref() {
            println!("{content}");
        } else if let Some(reasoning) = choice.message.reasoning_content.as_deref() {
            println!("{reasoning}");
        } else {
            println!("No visible assistant content returned.");
        }
    }

    Ok(())
}
```

## API Reference

### Support Matrix

| Surface | Status | Entry point |
|---------|--------|-------------|
| Chat completions | Supported | `client.chat().completions(...)` |
| Chat streaming | Supported | `client.chat().completions_stream(...)` |
| Translation | Supported | `client.text().translate(...)` |
| Transliteration | Supported | `client.text().transliterate(...)` |
| Language identification | Supported | `client.text().identify_language(...)` |
| Speech-to-text REST | Supported | `client.speech_to_text().transcribe(...).send()` |
| Speech-to-text streaming | Supported | `client.speech_to_text().stream().connect()` |
| Speech-to-text batch | Supported | `client.speech_to_text_batch()` |
| Speech-to-text translate REST | Supported | `client.speech_to_text_translate().translate(...).send()` |
| Speech-to-text translate streaming | Supported | `client.speech_to_text_translate().stream().connect()` |
| Speech-to-text translate batch | Supported | `client.speech_to_text_translate_batch()` |
| Text-to-speech REST | Supported | `client.text_to_speech().convert(...)` |
| Text-to-speech streaming | Supported | `client.text_to_speech().stream().connect()` |
| Document Intelligence | Supported | `client.document_intelligence()` |

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
    model: ChatModel::Sarvam30b,
    temperature: Some(0.2),
    reasoning_effort: Some(ReasoningEffort::Low),
    max_tokens: Some(256),
    ..Default::default()
};

let response = client.chat().completions(request).await?;
```

Available models: `Sarvam105b`, `Sarvam30b`, `SarvamM`.
For new workloads, prefer `Sarvam30b` or `Sarvam105b`.

#### Streaming Chat

```rust
let mut stream = client.chat().completions_stream(request).await?;

while let Some(result) = stream.next().await {
    let chunk = result?;
    for choice in &chunk.choices {
        if let Some(content) = choice.delta.content.as_deref() {
            print!("{content}");
        } else if let Some(reasoning) = choice.delta.reasoning_content.as_deref() {
            print!("{reasoning}");
        }
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
    .model(SpeechToTextModel::SaarasV3)
    .mode(SttMode::Transcribe)
    .language_enum(SpeechToTextLanguage::Unknown)
    .send()
    .await?;

println!("{}", response.transcript);
```

### Speech-to-Text Translate

Transcribe audio and translate to English in one step using the `/speech-to-text` endpoint:

```rust
use sarvam::types::speech_to_text::*;

let response = client
    .speech_to_text()
    .transcribe("audio.wav")
    .model(SpeechToTextModel::SaarasV3)
    .mode(SttMode::Translate)
    .language_enum(SpeechToTextLanguage::Unknown)
    .send()
    .await?;

println!("{}", response.transcript);
```

If you need the dedicated endpoint instead of `mode=translate`:

```rust
use sarvam::types::speech_to_text::*;

let response = client
    .speech_to_text_translate()
    .translate("audio.wav")
    .model(SpeechToTextTranslateModel::SaarasV2_5)
    .send()
    .await?;

println!("{}", response.transcript);
```

#### Streaming STT (WebSocket)

```rust
use sarvam::streaming::{
    SttAudioEncoding, SttMessage, SttStreamingModel, WebSocketSampleRate,
};
use sarvam::types::{SpeechToTextLanguage, SttMode};

let audio = std::fs::read("audio.wav")?;

let mut stream = client
    .speech_to_text()
    .stream()
    .model(SttStreamingModel::SaarasV3)
    .mode(SttMode::Transcribe)
    .language_code(SpeechToTextLanguage::EnIn)
    .flush_signal(true)
    .connect()
    .await?;

stream
    .send_audio(audio, SttAudioEncoding::AudioWav, WebSocketSampleRate::Hz16000)
    .await?;
stream.flush().await?;

while let Some(message) = stream.next().await {
    match message? {
        SttMessage::Transcript(transcript) => {
            if let Some(text) = transcript.text() {
                println!("{text}");
                break;
            }
        }
        SttMessage::Event(event) => println!("event={}", event.event_type),
        SttMessage::Error(err) => eprintln!("{:?}", err),
        SttMessage::Raw(raw) => println!("{}", raw),
    }
}
```

#### Batch STT

```rust
use std::time::Duration;
use sarvam::types::{
    CreateSpeechBatchJobRequest, SpeechBatchModel, SpeechBatchUploadUrlsRequest,
    SpeechToTextBatchJobParameters, SttMode,
};

let batch = client.speech_to_text_batch();

let job = batch
    .create_job(CreateSpeechBatchJobRequest {
        job_parameters: SpeechToTextBatchJobParameters {
            model: Some(SpeechBatchModel::SaarasV3),
            mode: Some(SttMode::Transcribe),
            ..Default::default()
        },
        callback: None,
    })
    .await?;

let upload_urls = batch
    .get_upload_urls(SpeechBatchUploadUrlsRequest {
        job_id: job.job_id.clone(),
        files: vec!["audio.wav".into()],
    })
    .await?;

println!("Upload via presigned URLs: {}", upload_urls.upload_urls.len());
batch.start_job(&job.job_id).await?;
let status = batch
    .wait_until_terminal(&job.job_id, Duration::from_secs(2))
    .await?;
println!("Final batch state: {:?}", status.job_state);
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

### Document Intelligence

```rust
use std::time::Duration;
use sarvam::types::{
    CreateDocumentIntelligenceJobRequest, DocumentIntelligenceJobParameters,
    DocumentIntelligenceLanguage, DocumentOutputFormat, DocumentUploadUrlsRequest,
};

let docs = client.document_intelligence();

let job = docs
    .create_job(CreateDocumentIntelligenceJobRequest {
        job_parameters: Some(DocumentIntelligenceJobParameters {
            language: Some(DocumentIntelligenceLanguage::EnIn),
            output_format: Some(DocumentOutputFormat::Markdown),
            ..Default::default()
        }),
        callback: None,
    })
    .await?;

let upload_urls = docs
    .get_upload_urls(DocumentUploadUrlsRequest {
        job_id: job.job_id.clone(),
        files: vec!["document.pdf".into()],
    })
    .await?;

println!("Upload via presigned URLs: {}", upload_urls.upload_urls.len());
docs.start_job(&job.job_id).await?;

let status = docs
    .wait_until_terminal(&job.job_id, Duration::from_secs(2))
    .await?;

if status.job_state.is_success_like() {
    let downloads = docs.get_download_urls(&job.job_id).await?;
    println!("Result files: {}", downloads.download_urls.len());
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
| `streaming` | Yes   | SSE chat streaming plus STT/STTT/TTS WebSocket clients |

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
| `speech_to_text_translate` | Audio transcription + English translation using `mode=translate` |
| `speech_to_text_stream` | Streaming speech-to-text via WebSocket |
| `speech_to_text_translate_stream` | Streaming speech-to-text translate via WebSocket |
| `speech_to_text_batch` | Batch speech-to-text job workflow |
| `speech_to_text_translate_batch` | Batch speech-to-text translate job workflow |
| `text_to_speech` | Text-to-audio conversion |
| `text_to_speech_stream` | Streaming text-to-audio via WebSocket |
| `document_intelligence` | Document Intelligence job workflow |

Run an example:

```bash
export SARVAM_API_KEY='your-key'
cargo run --example chat
```

Examples that need an audio file also read `AUDIO_FILE`:

```bash
export SARVAM_API_KEY='your-key'
export AUDIO_FILE='audio.wav'
cargo run --example speech_to_text
```

## License

[License: MIT](./License.md)
