use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::job::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentIntelligenceLanguage {
    #[serde(rename = "hi-IN")]
    HiIn,
    #[serde(rename = "en-IN")]
    EnIn,
    #[serde(rename = "bn-IN")]
    BnIn,
    #[serde(rename = "gu-IN")]
    GuIn,
    #[serde(rename = "kn-IN")]
    KnIn,
    #[serde(rename = "ml-IN")]
    MlIn,
    #[serde(rename = "mr-IN")]
    MrIn,
    #[serde(rename = "or-IN")]
    OrIn,
    #[serde(rename = "pa-IN")]
    PaIn,
    #[serde(rename = "ta-IN")]
    TaIn,
    #[serde(rename = "te-IN")]
    TeIn,
    #[serde(rename = "ur-IN")]
    UrIn,
    #[serde(rename = "as-IN")]
    AsIn,
    #[serde(rename = "bodo-IN")]
    BodoIn,
    #[serde(rename = "doi-IN")]
    DoiIn,
    #[serde(rename = "ks-IN")]
    KsIn,
    #[serde(rename = "kok-IN")]
    KokIn,
    #[serde(rename = "mai-IN")]
    MaiIn,
    #[serde(rename = "mni-IN")]
    MniIn,
    #[serde(rename = "ne-IN")]
    NeIn,
    #[serde(rename = "sa-IN")]
    SaIn,
    #[serde(rename = "sat-IN")]
    SatIn,
    #[serde(rename = "sd-IN")]
    SdIn,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DocumentOutputFormat {
    #[serde(rename = "html")]
    Html,
    #[serde(rename = "md")]
    Markdown,
    #[serde(rename = "json")]
    Json,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocumentIntelligenceJobParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<DocumentIntelligenceLanguage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<DocumentOutputFormat>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDocumentIntelligenceJobRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_parameters: Option<DocumentIntelligenceJobParameters>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback: Option<BulkJobCallback>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentIntelligenceJobAcceptedResponse {
    pub job_id: String,
    pub storage_container_type: StorageContainerType,
    #[serde(default)]
    pub job_parameters: Option<DocumentIntelligenceJobParameters>,
    pub job_state: JobState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploadUrlsRequest {
    pub job_id: String,
    pub files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentUploadUrlsResponse {
    pub job_id: String,
    pub job_state: JobState,
    pub upload_urls: HashMap<String, PresignedFileAccess>,
    pub storage_container_type: StorageContainerType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentJobStatusResponse {
    pub job_id: String,
    pub job_state: JobState,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
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
    #[serde(default)]
    pub error_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDownloadUrlsResponse {
    pub job_id: String,
    pub job_state: JobState,
    pub storage_container_type: StorageContainerType,
    pub download_urls: HashMap<String, PresignedFileAccess>,
    #[serde(default)]
    pub error_code: Option<String>,
    #[serde(default)]
    pub error_message: Option<String>,
}
