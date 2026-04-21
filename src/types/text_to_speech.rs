use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize)]
pub struct TextToSpeechRequest {
    pub text: String,
    pub target_language_code: TextToSpeechLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker: Option<TextToSpeechSpeaker>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pitch: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pace: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loudness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speech_sample_rate: Option<SpeechSampleRate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_preprocessing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<TextToSpeechModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_audio_codec: Option<TextToSpeechOutputAudioCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dict_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_cached_responses: Option<bool>,
}

impl Default for TextToSpeechRequest {
    fn default() -> Self {
        Self {
            text: String::new(),
            target_language_code: TextToSpeechLanguage::EnIn,
            speaker: None,
            pitch: None,
            pace: None,
            loudness: None,
            speech_sample_rate: None,
            enable_preprocessing: None,
            model: None,
            output_audio_codec: None,
            temperature: None,
            dict_id: None,
            enable_cached_responses: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct TextToSpeechResponse {
    pub request_id: Option<String>,
    pub audios: Vec<String>,
}
