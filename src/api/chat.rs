use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::chat::*;

pub struct ChatApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

impl ChatApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn completions(&self, request: ChatCompletionRequest) -> Result<ChatCompletionResponse> {
        let url = format!("{}/v1/chat/completions", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .header(
                "api-subscription-key",
                &self.config.api_subscription_key,
            )
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_subscription_key),
            )
            .json(&request)
            .timeout(self.config.timeout)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SarvamError::from_response(status, &body));
        }

        response
            .json::<ChatCompletionResponse>()
            .await
            .map_err(SarvamError::from)
    }

    #[cfg(feature = "streaming")]
    pub async fn completions_stream(
        &self,
        mut request: ChatCompletionRequest,
    ) -> Result<crate::streaming::ChatStream> {
        use crate::streaming::ChatStream;

        request.stream = Some(true);
        let url = format!("{}/v1/chat/completions", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .header(
                "api-subscription-key",
                &self.config.api_subscription_key,
            )
            .header(
                "Authorization",
                format!("Bearer {}", self.config.api_subscription_key),
            )
            .json(&request)
            .timeout(self.config.timeout)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(SarvamError::from_response(status, &body));
        }

        Ok(ChatStream::new(response))
    }
}
