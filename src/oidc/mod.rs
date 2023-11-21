pub mod authorize;

#[allow(dead_code)]
#[derive(Debug, thiserror::Error)]
pub enum OidcError {
    #[error("Access denied")]
    AccessDenied,

    #[error("Invalid client")]
    InvalidClient,

    #[error("Invalid grant")]
    InvalidRequest,

    #[error("Invalid scope")]
    InvalidScope,

    #[error("Invalid token")]
    ServerError,

    #[error("Temporarily unavailable")]
    TemporarilyUnavailable,

    #[error("Unauthorized client")]
    UnauthorizedClient,

    #[error("Unsupported grant type")]
    UnsupportedResponseType,

    #[error("Unspecified error: {0}")]
    Unspecified(String),
}
