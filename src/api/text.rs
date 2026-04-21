use crate::config::SarvamConfig;
use crate::error::{Result, SarvamError};
use crate::types::translate::*;
use crate::types::transliterate::*;

pub struct TextApi {
    config: SarvamConfig,
    client: reqwest::Client,
}

impl TextApi {
    pub(crate) fn new(config: SarvamConfig, client: reqwest::Client) -> Self {
        Self { config, client }
    }

    pub async fn translate(&self, request: TranslationRequest) -> Result<TranslationResponse> {
        let url = format!("{}/translate", self.config.base_url);

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
            .json::<TranslationResponse>()
            .await
            .map_err(SarvamError::from)
    }

    pub async fn transliterate(
        &self,
        request: TransliterationRequest,
    ) -> Result<TransliterationResponse> {
        let url = format!("{}/transliterate", self.config.base_url);

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
            .json::<TransliterationResponse>()
            .await
            .map_err(SarvamError::from)
    }

    pub async fn identify_language(
        &self,
        request: LanguageIdentificationRequest,
    ) -> Result<LanguageIdentificationResponse> {
        let url = format!("{}/text-lid", self.config.base_url);

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
            .json::<LanguageIdentificationResponse>()
            .await
            .map_err(SarvamError::from)
    }
}
