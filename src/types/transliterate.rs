use serde::{Deserialize, Serialize};

use super::common::*;

#[derive(Debug, Clone, Serialize)]
pub struct TransliterationRequest {
    pub input: String,
    pub source_language_code: TransliterateSourceLanguage,
    pub target_language_code: TransliterateTargetLanguage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numerals_format: Option<NumeralsFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_form_numerals_language: Option<SpokenFormNumeralsFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spoken_form: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TransliterationResponse {
    pub request_id: Option<String>,
    pub transliterated_text: String,
    pub source_language_code: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LanguageIdentificationRequest {
    pub input: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LanguageIdentificationResponse {
    pub request_id: Option<String>,
    pub language_code: Option<String>,
    pub script_code: Option<String>,
}
