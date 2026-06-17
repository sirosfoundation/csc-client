//! Error types for the CSC client.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, CscError>;

#[derive(Debug, Error)]
pub enum CscError {
    /// HTTP transport error.
    #[error("HTTP error: {0}")]
    Http(String),

    /// CSC API returned an error response.
    #[error("CSC API error {status}: {error} — {error_description}")]
    Api {
        status: u16,
        error: String,
        error_description: String,
    },

    /// Response could not be deserialized.
    #[error("invalid response: {0}")]
    InvalidResponse(String),

    /// DPoP signer error.
    #[error("DPoP signing failed: {0}")]
    DPop(String),

    /// The credential requires authorization that was not provided.
    #[error("authorization required: {0}")]
    AuthorizationRequired(String),
}

impl From<reqwest::Error> for CscError {
    fn from(e: reqwest::Error) -> Self {
        CscError::Http(e.to_string())
    }
}
