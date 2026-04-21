use std::path::Path;

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::speech_to_text::*;

pub struct SpeechToTextTranslateApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

pub struct SttTranslateRequestBuilder {
    config: SarvamConfig,
    client: reqwest::Client,
    file_path: String,
    model: Option<SpeechToTextTranslateModel>,
    prompt: Option<String>,
    input_audio_codec: Option<InputAudioCodec>,
}

impl SttTranslateRequestBuilder {
    pub fn model(mut self, model: SpeechToTextTranslateModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn prompt(mut self, prompt: impl Into<String>) -> Self {
        self.prompt = Some(prompt.into());
        self
    }

    pub fn input_audio_codec(mut self, codec: InputAudioCodec) -> Self {
        self.input_audio_codec = Some(codec);
        self
    }

    pub async fn send(self) -> Result<SpeechToTextTranslateResponse> {
        let file_name = Path::new(&self.file_path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "audio.wav".to_string());

        let file_bytes = tokio::fs::read(&self.file_path).await.map_err(|e| {
            SarvamError::Custom(format!("Failed to read file '{}': {}", self.file_path, e))
        })?;

        let file_part = reqwest::multipart::Part::bytes(file_bytes)
            .file_name(file_name)
            .mime_str("application/octet-stream")
            .map_err(|e| SarvamError::Custom(e.to_string()))?;

        let mut form = reqwest::multipart::Form::new().part("file", file_part);

        if let Some(model) = &self.model {
            let val = serde_json::to_value(model)
                .map_err(|e| SarvamError::Custom(e.to_string()))?
                .as_str()
                .ok_or_else(|| SarvamError::Custom("Invalid model value".into()))?
                .to_string();
            form = form.text("model", val);
        }

        if let Some(prompt) = self.prompt {
            form = form.text("prompt", prompt);
        }

        if let Some(codec) = &self.input_audio_codec {
            let val = serde_json::to_value(codec)
                .map_err(|e| SarvamError::Custom(e.to_string()))?
                .as_str()
                .ok_or_else(|| SarvamError::Custom("Invalid codec value".into()))?
                .to_string();
            form = form.text("input_audio_codec", val);
        }

        let url = format!("{}/speech-to-text-translate", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .multipart(form)
            .timeout(self.config.timeout)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SarvamError::from_response(status, &body));
        }

        response
            .json::<SpeechToTextTranslateResponse>()
            .await
            .map_err(SarvamError::from)
    }
}

impl SpeechToTextTranslateApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub fn translate(&self, file_path: impl Into<String>) -> SttTranslateRequestBuilder {
        SttTranslateRequestBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            file_path: file_path.into(),
            model: None,
            prompt: None,
            input_audio_codec: None,
        }
    }

    #[cfg(feature = "streaming")]
    pub fn stream(&self) -> crate::streaming::SttTranslateStreamBuilder {
        crate::streaming::SttTranslateStreamBuilder::new(self.config.clone())
    }
}
