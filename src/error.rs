//! Error types for rbuster

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RbusterError {
    #[error("Failed to read wordlist: {0}")]
    WordlistError(#[from] std::io::Error),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("DNS resolution failed: {0}")]
    DnsError(String),

    #[error("Invalid URL: {0}")]
    UrlError(#[from] url::ParseError),

    #[error("TFTP error: {0}")]
    TftpError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Channel error: {0}")]
    ChannelError(String),
}

pub type Result<T> = std::result::Result<T, RbusterError>;
