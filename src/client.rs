use crate::api::chat::ChatApi;
use crate::api::document_intelligence::DocumentIntelligenceApi;
use crate::api::speech_to_text::SpeechToTextApi;
use crate::api::speech_to_text_batch::{SpeechToTextBatchApi, SpeechToTextTranslateBatchApi};
use crate::api::speech_to_text_translate::SpeechToTextTranslateApi;
use crate::api::text::TextApi;
use crate::api::text_to_speech::TextToSpeechApi;
use crate::config::SarvamConfig;

#[derive(Clone)]
pub struct SarvamClient {
    config: SarvamConfig,
    http: reqwest::Client,
}

impl SarvamClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::from_config(SarvamConfig::new(api_key))
    }

    pub fn from_config(config: SarvamConfig) -> Self {
        let http = reqwest::Client::builder()
            .timeout(config.timeout)
            .build()
            .expect("Failed to build HTTP client");
        Self { config, http }
    }

    pub fn chat(&self) -> ChatApi {
        ChatApi::new(self.config.clone(), self.http.clone())
    }

    pub fn text(&self) -> TextApi {
        TextApi::new(self.config.clone(), self.http.clone())
    }

    pub fn speech_to_text(&self) -> SpeechToTextApi {
        SpeechToTextApi::new(self.config.clone(), self.http.clone())
    }

    pub fn speech_to_text_batch(&self) -> SpeechToTextBatchApi {
        SpeechToTextBatchApi::new(self.config.clone(), self.http.clone())
    }

    pub fn speech_to_text_translate(&self) -> SpeechToTextTranslateApi {
        SpeechToTextTranslateApi::new(self.config.clone(), self.http.clone())
    }

    pub fn speech_to_text_translate_batch(&self) -> SpeechToTextTranslateBatchApi {
        SpeechToTextTranslateBatchApi::new(self.config.clone(), self.http.clone())
    }

    pub fn text_to_speech(&self) -> TextToSpeechApi {
        TextToSpeechApi::new(self.config.clone(), self.http.clone())
    }

    pub fn document_intelligence(&self) -> DocumentIntelligenceApi {
        DocumentIntelligenceApi::new(self.config.clone(), self.http.clone())
    }
}
