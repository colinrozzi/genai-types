use std::error::Error;
use std::fmt;

/// Error type for Anthropic API operations
#[derive(Debug)]
pub enum GenAiError {
    /// HTTP request failed
    HttpError(String),

    /// Failed to serialize/deserialize JSON
    JsonError(String),

    /// API returned an error
    ApiError { status: u16, message: String },

    /// Unexpected response format
    InvalidResponse(String),

    /// Rate limit exceeded
    RateLimitExceeded { retry_after: Option<u64> },

    /// Authentication error
    AuthenticationError(String),
}

impl fmt::Display for GenAiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GenAiError::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            GenAiError::JsonError(msg) => write!(f, "JSON error: {}", msg),
            GenAiError::ApiError { status, message } => {
                write!(f, "API error ({}): {}", status, message)
            }
            GenAiError::InvalidResponse(msg) => write!(f, "Invalid response: {}", msg),
            GenAiError::RateLimitExceeded { retry_after } => {
                if let Some(seconds) = retry_after {
                    write!(f, "Rate limit exceeded. Retry after {} seconds", seconds)
                } else {
                    write!(f, "Rate limit exceeded")
                }
            }
            GenAiError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

impl Error for GenAiError {}

impl From<serde_json::Error> for GenAiError {
    fn from(error: serde_json::Error) -> Self {
        GenAiError::JsonError(error.to_string())
    }
}
