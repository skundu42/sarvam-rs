use std::time::Duration;

const DEFAULT_BASE_URL: &str = "https://api.sarvam.ai";
const DEFAULT_TIMEOUT_SECS: u64 = 60;

#[derive(Clone, Debug)]
pub struct SarvamConfig {
    pub api_subscription_key: String,
    pub base_url: String,
    pub timeout: Duration,
}

impl SarvamConfig {
    pub fn new(api_subscription_key: impl Into<String>) -> Self {
        Self {
            api_subscription_key: api_subscription_key.into(),
            base_url: DEFAULT_BASE_URL.to_string(),
            timeout: Duration::from_secs(DEFAULT_TIMEOUT_SECS),
        }
    }

    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = url.into();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
}
