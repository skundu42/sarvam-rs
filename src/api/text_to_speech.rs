use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::text_to_speech::*;

pub struct TextToSpeechApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

impl TextToSpeechApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn convert(&self, request: TextToSpeechRequest) -> Result<TextToSpeechResponse> {
        let url = format!("{}/text-to-speech", self.config.base_url);

        let response = self
            .client
            .post(&url)
            .header("api-subscription-key", &self.config.api_subscription_key)
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
            .json::<TextToSpeechResponse>()
            .await
            .map_err(SarvamError::from)
    }

    #[cfg(feature = "streaming")]
    pub fn stream(&self) -> crate::streaming::TtsStreamBuilder {
        crate::streaming::TtsStreamBuilder::new(self.config.clone())
    }
}
