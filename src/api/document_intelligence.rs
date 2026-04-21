use std::time::Duration;

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::document_intelligence::*;

pub struct DocumentIntelligenceApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

impl DocumentIntelligenceApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn create_job(
        &self,
        request: CreateDocumentIntelligenceJobRequest,
    ) -> Result<DocumentIntelligenceJobAcceptedResponse> {
        let url = format!("{}/doc-digitization/job/v1", self.config.base_url);
        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .json(&request)
            .timeout(self.config.timeout)
            .send()
            .await?;
        parse_json_response(response).await
    }

    pub async fn get_upload_urls(
        &self,
        request: DocumentUploadUrlsRequest,
    ) -> Result<DocumentUploadUrlsResponse> {
        let url = format!(
            "{}/doc-digitization/job/v1/upload-files",
            self.config.base_url
        );
        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .json(&request)
            .timeout(self.config.timeout)
            .send()
            .await?;
        parse_json_response(response).await
    }

    pub async fn start_job(&self, job_id: &str) -> Result<DocumentJobStatusResponse> {
        let url = format!(
            "{}/doc-digitization/job/v1/{}/start",
            self.config.base_url, job_id
        );
        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .json(&serde_json::json!({}))
            .timeout(self.config.timeout)
            .send()
            .await?;
        parse_json_response(response).await
    }

    pub async fn get_status(&self, job_id: &str) -> Result<DocumentJobStatusResponse> {
        let url = format!(
            "{}/doc-digitization/job/v1/{}/status",
            self.config.base_url, job_id
        );
        let response = self
            .client
            .get(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .timeout(self.config.timeout)
            .send()
            .await?;
        parse_json_response(response).await
    }

    pub async fn get_download_urls(&self, job_id: &str) -> Result<DocumentDownloadUrlsResponse> {
        let url = format!(
            "{}/doc-digitization/job/v1/{}/download-files",
            self.config.base_url, job_id
        );
        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
            .json(&serde_json::json!({}))
            .timeout(self.config.timeout)
            .send()
            .await?;
        parse_json_response(response).await
    }

    pub async fn wait_until_terminal(
        &self,
        job_id: &str,
        poll_interval: Duration,
    ) -> Result<DocumentJobStatusResponse> {
        loop {
            let status = self.get_status(job_id).await?;
            if status.job_state.is_terminal() {
                return Ok(status);
            }
            tokio::time::sleep(poll_interval).await;
        }
    }
}

async fn parse_json_response<T: serde::de::DeserializeOwned>(
    response: reqwest::Response,
) -> Result<T> {
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        return Err(SarvamError::from_response(status, &body));
    }

    response.json::<T>().await.map_err(SarvamError::from)
}
