use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StorageContainerType {
    Azure,
    Local,
    Google,
    #[serde(rename = "Azure_V1")]
    AzureV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobState {
    Accepted,
    Pending,
    Running,
    Completed,
    #[serde(rename = "PartiallyCompleted")]
    PartiallyCompleted,
    Failed,
}

impl JobState {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobState::Completed | JobState::PartiallyCompleted | JobState::Failed
        )
    }

    pub fn is_success_like(&self) -> bool {
        matches!(self, JobState::Completed | JobState::PartiallyCompleted)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkJobCallback {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FileMetadata {
    #[serde(rename = "contentType", skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(rename = "fileSizeBytes", skip_serializing_if = "Option::is_none")]
    pub file_size_bytes: Option<u64>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PresignedFileAccess {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_metadata: Option<FileMetadata>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl PresignedFileAccess {
    pub fn url(&self) -> Option<&str> {
        self.file_url
            .as_deref()
            .or(self.upload_url.as_deref())
            .or(self.url.as_deref())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFileRef {
    pub file_name: String,
    pub file_id: String,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobDetail {
    #[serde(default)]
    pub inputs: Vec<JobFileRef>,
    #[serde(default)]
    pub outputs: Vec<JobFileRef>,
    pub state: String,
    #[serde(default)]
    pub error_message: Option<String>,
    #[serde(default)]
    pub exception_name: Option<String>,
    #[serde(default)]
    pub total_pages: Option<u32>,
    #[serde(default)]
    pub pages_processed: Option<u32>,
    #[serde(default)]
    pub pages_succeeded: Option<u32>,
    #[serde(default)]
    pub pages_failed: Option<u32>,
    #[serde(default)]
    pub page_errors: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

impl JobDetail {
    pub fn primary_input_file_name(&self) -> Option<&str> {
        self.inputs.first().map(|input| input.file_name.as_str())
    }
}
