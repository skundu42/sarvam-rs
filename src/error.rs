use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, SarvamError>;

#[derive(Debug, thiserror::Error)]
pub enum SarvamError {
    #[error("API error ({code}): {message}")]
    ApiError {
        code: ErrorCode,
        message: String,
        request_id: Option<String>,
    },

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("{0}")]
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, thiserror::Error)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    #[error("invalid request")]
    InvalidRequestError,
    #[error("internal server error")]
    InternalServerError,
    #[error("unprocessable entity")]
    UnprocessableEntityError,
    #[error("insufficient quota")]
    InsufficientQuotaError,
    #[error("invalid API key")]
    InvalidApiKeyError,
    #[error("authentication error")]
    AuthenticationError,
    #[error("rate limit exceeded")]
    RateLimitExceededError,
    #[error("not found")]
    NotFoundError,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorBody {
    pub error: ErrorDetails,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ErrorDetails {
    pub message: String,
    pub code: ErrorCode,
    pub request_id: Option<String>,
}

impl SarvamError {
    pub fn from_response(status: reqwest::StatusCode, body: &str) -> Self {
        if let Ok(err_body) = serde_json::from_str::<ErrorBody>(body) {
            SarvamError::ApiError {
                code: err_body.error.code,
                message: err_body.error.message,
                request_id: err_body.error.request_id,
            }
        } else {
            SarvamError::Custom(format!("HTTP {}: {}", status, body))
        }
    }
}
