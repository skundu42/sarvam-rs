use std::time::Duration;

use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::speech_to_text_batch::*;

pub struct SpeechToTextBatchApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

pub struct SpeechToTextTranslateBatchApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

impl SpeechToTextBatchApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn create_job(
        &self,
        request: CreateSpeechBatchJobRequest<SpeechToTextBatchJobParameters>,
    ) -> Result<SpeechBatchJobAcceptedResponse<SpeechToTextBatchJobParameters>> {
        let url = format!("{}/speech-to-text/job/v1", self.config.base_url);
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
        request: SpeechBatchUploadUrlsRequest,
    ) -> Result<SpeechBatchUploadUrlsResponse> {
        let url = format!(
            "{}/speech-to-text/job/v1/upload-files",
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

    pub async fn start_job(&self, job_id: &str) -> Result<SpeechBatchJobStatusResponse> {
        let url = format!(
            "{}/speech-to-text/job/v1/{}/start",
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

    pub async fn get_status(&self, job_id: &str) -> Result<SpeechBatchJobStatusResponse> {
        let url = format!(
            "{}/speech-to-text/job/v1/{}/status",
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

    pub async fn get_download_urls(
        &self,
        request: SpeechBatchDownloadUrlsRequest,
    ) -> Result<SpeechBatchDownloadUrlsResponse> {
        let url = format!(
            "{}/speech-to-text/job/v1/download-files",
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

    pub async fn wait_until_terminal(
        &self,
        job_id: &str,
        poll_interval: Duration,
    ) -> Result<SpeechBatchJobStatusResponse> {
        loop {
            let status = self.get_status(job_id).await?;
            if status.job_state.is_terminal() {
                return Ok(status);
            }
            tokio::time::sleep(poll_interval).await;
        }
    }
}

impl SpeechToTextTranslateBatchApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn create_job(
        &self,
        request: CreateSpeechBatchJobRequest<SpeechToTextTranslateBatchJobParameters>,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchJobAcceptedResponse<SpeechToTextTranslateBatchJobParameters>> {
        let url = batch_translate_url(
            &self.config.base_url,
            "/speech-to-text-translate/job/v1",
            ptu_id,
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

    pub async fn get_upload_urls(
        &self,
        request: SpeechBatchUploadUrlsRequest,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchUploadUrlsResponse> {
        let url = batch_translate_url(
            &self.config.base_url,
            "/speech-to-text-translate/job/v1/upload-files",
            ptu_id,
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

    pub async fn start_job(
        &self,
        job_id: &str,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchJobStatusResponse> {
        let url = batch_translate_url(
            &self.config.base_url,
            &format!("/speech-to-text-translate/job/v1/{job_id}/start"),
            ptu_id,
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

    pub async fn get_status(
        &self,
        job_id: &str,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchJobStatusResponse> {
        let url = batch_translate_url(
            &self.config.base_url,
            &format!("/speech-to-text-translate/job/v1/{job_id}/status"),
            ptu_id,
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

    pub async fn get_download_urls(
        &self,
        request: SpeechBatchDownloadUrlsRequest,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchDownloadUrlsResponse> {
        let url = batch_translate_url(
            &self.config.base_url,
            "/speech-to-text-translate/job/v1/download-files",
            ptu_id,
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

    pub async fn wait_until_terminal(
        &self,
        job_id: &str,
        poll_interval: Duration,
        ptu_id: Option<i64>,
    ) -> Result<SpeechBatchJobStatusResponse> {
        loop {
            let status = self.get_status(job_id, ptu_id).await?;
            if status.job_state.is_terminal() {
                return Ok(status);
            }
            tokio::time::sleep(poll_interval).await;
        }
    }
}

fn batch_translate_url(base_url: &str, path: &str, ptu_id: Option<i64>) -> String {
    match ptu_id {
        Some(ptu_id) => format!("{base_url}{path}?ptu_id={ptu_id}"),
        None => format!("{base_url}{path}"),
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
