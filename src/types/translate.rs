use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize)]
pub struct TranslationRequest {
    pub input: String,
    pub source_language_code: TranslateSourceLanguage,
    pub target_language_code: TranslateTargetLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speaker_gender: Option<SpeakerGender>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<TranslateMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<TranslateModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_script: Option<TransliterateMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerals_format: Option<NumeralsFormat>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TranslationResponse {
    pub request_id: Option<String>,
    pub translated_text: String,
    pub source_language_code: String,
}
