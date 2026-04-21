use std::path::Path;

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::speech_to_text::*;

pub struct SpeechToTextApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

pub struct SpeechToTextRequestBuilder {
    config: SarvamConfig,
    client: reqwest::Client,
    file_path: String,
    model: Option<SpeechToTextModel>,
    mode: Option<SttMode>,
    language_code: Option<SpeechToTextLanguage>,
    input_audio_codec: Option<InputAudioCodec>,
}

impl SpeechToTextRequestBuilder {
    pub fn model(mut self, model: SpeechToTextModel) -> Self {
        self.model = Some(model);
        self
    }

    pub fn mode(mut self, mode: SttMode) -> Self {
        self.mode = Some(mode);
        self
    }

    pub fn language_code(mut self, lang: impl Into<String>) -> Self {
        self.language_code = Some(SpeechToTextLanguage::from_str(&lang.into()));
        self
    }

    pub fn language_enum(mut self, lang: SpeechToTextLanguage) -> Self {
        self.language_code = Some(lang);
        self
    }

    pub fn input_audio_codec(mut self, codec: InputAudioCodec) -> Self {
        self.input_audio_codec = Some(codec);
        self
    }

    pub async fn send(self) -> Result<SpeechToTextResponse> {
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

        if let Some(mode) = &self.mode {
            let val = serde_json::to_value(mode)
                .map_err(|e| SarvamError::Custom(e.to_string()))?
                .as_str()
                .ok_or_else(|| SarvamError::Custom("Invalid mode value".into()))?
                .to_string();
            form = form.text("mode", val);
        }

        if let Some(lang) = &self.language_code {
            let val = serde_json::to_value(lang)
                .map_err(|e| SarvamError::Custom(e.to_string()))?
                .as_str()
                .ok_or_else(|| SarvamError::Custom("Invalid language value".into()))?
                .to_string();
            form = form.text("language_code", val);
        }

        if let Some(codec) = &self.input_audio_codec {
            let val = serde_json::to_value(codec)
                .map_err(|e| SarvamError::Custom(e.to_string()))?
                .as_str()
                .ok_or_else(|| SarvamError::Custom("Invalid codec value".into()))?
                .to_string();
            form = form.text("input_audio_codec", val);
        }

        let url = format!("{}/speech-to-text", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .header(
                "api-subscription-key",
                &self.config.api_subscription_key,
            )
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
            .json::<SpeechToTextResponse>()
            .await
            .map_err(SarvamError::from)
    }
}

impl SpeechToTextApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub fn transcribe(&self, file_path: impl Into<String>) -> SpeechToTextRequestBuilder {
        SpeechToTextRequestBuilder {
            config: self.config.clone(),
            client: self.client.clone(),
            file_path: file_path.into(),
            model: None,
            mode: None,
            language_code: None,
            input_audio_codec: None,
        }
    }
}

impl SpeechToTextLanguage {
    pub fn from_str(s: &str) -> Self {
        match s {
            "unknown" => SpeechToTextLanguage::Unknown,
            "hi-IN" => SpeechToTextLanguage::HiIn,
            "bn-IN" => SpeechToTextLanguage::BnIn,
            "kn-IN" => SpeechToTextLanguage::KnIn,
            "ml-IN" => SpeechToTextLanguage::MlIn,
            "mr-IN" => SpeechToTextLanguage::MrIn,
            "od-IN" => SpeechToTextLanguage::OdIn,
            "pa-IN" => SpeechToTextLanguage::PaIn,
            "ta-IN" => SpeechToTextLanguage::TaIn,
            "te-IN" => SpeechToTextLanguage::TeIn,
            "en-IN" => SpeechToTextLanguage::EnIn,
            "gu-IN" => SpeechToTextLanguage::GuIn,
            "as-IN" => SpeechToTextLanguage::AsIn,
            "ur-IN" => SpeechToTextLanguage::UrIn,
            "ne-IN" => SpeechToTextLanguage::NeIn,
            "kok-IN" => SpeechToTextLanguage::KokIn,
            "ks-IN" => SpeechToTextLanguage::KsIn,
            "sd-IN" => SpeechToTextLanguage::SdIn,
            "sa-IN" => SpeechToTextLanguage::SaIn,
            "sat-IN" => SpeechToTextLanguage::SatIn,
            "mni-IN" => SpeechToTextLanguage::MniIn,
            "brx-IN" => SpeechToTextLanguage::BrxIn,
            "mai-IN" => SpeechToTextLanguage::MaiIn,
            "doi-IN" => SpeechToTextLanguage::DoiIn,
            _ => SpeechToTextLanguage::Unknown,
        }
    }
}
