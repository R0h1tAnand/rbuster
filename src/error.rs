//! Custom error types for rbuster

use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum RbusterError {
    #[error("Failed to read wordlist: {0}")]
    WordlistError(#[from] std::io::Error),

    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("DNS resolution failed: {0}")]
    DnsError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, RbusterError>;
