use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::job::*;
use super::speech_to_text::{InputAudioCodec, SpeechToTextLanguage, SttMode};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SpeechBatchModel {
    #[serde(rename = "saaras:v3")]
    SaarasV3,
    #[serde(rename = "saaras:v2.5")]
    SaarasV2_5,
    #[serde(rename = "saarika:v2.5")]
    SaarikaV2_5,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechToTextBatchJobParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SpeechBatchModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SttMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<SpeechToTextLanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_codec: Option<InputAudioCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_diarization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<u8>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpeechToTextTranslateBatchJobParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SpeechBatchModel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<SttMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_audio_codec: Option<InputAudioCodec>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_diarization: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_speakers: Option<u8>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSpeechBatchJobRequest<T> {
    pub job_parameters: T,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<BulkJobCallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchJobAcceptedResponse<T> {
    pub job_id: String,
    pub storage_container_type: StorageContainerType,
    pub job_parameters: T,
    pub job_state: JobState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchUploadUrlsRequest {
    pub job_id: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchUploadUrlsResponse {
    pub job_id: String,
    pub job_state: JobState,
    pub upload_urls: HashMap<String, PresignedFileAccess>,
    pub storage_container_type: StorageContainerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchDownloadUrlsRequest {
    pub job_id: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchDownloadUrlsResponse {
    pub job_id: String,
    pub job_state: JobState,
    pub download_urls: HashMap<String, PresignedFileAccess>,
    pub storage_container_type: StorageContainerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechBatchJobStatusResponse {
    pub job_state: JobState,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
    pub job_id: String,
    pub storage_container_type: StorageContainerType,
    #[serde(default)]
    pub total_files: u32,
    #[serde(default)]
    pub successful_files_count: u32,
    #[serde(default)]
    pub failed_files_count: u32,
    #[serde(default)]
    pub error_message: String,
    #[serde(default)]
    pub job_details: Vec<JobDetail>,
}

impl SpeechBatchJobStatusResponse {
    pub fn successful_output_files(&self) -> Vec<String> {
        self.job_details
            .iter()
            .filter(|detail| detail.state.eq_ignore_ascii_case("success"))
            .flat_map(|detail| detail.outputs.iter().map(|output| output.file_name.clone()))
            .collect()
    }
}
