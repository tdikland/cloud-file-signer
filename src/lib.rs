use std::fmt::{Display, Formatter};

pub mod aws;
pub mod azure;
pub mod gcp;

#[derive(Debug)]
pub struct PresignedUrl {
    url: String,
    expires: String,
}

impl PresignedUrl {
    fn new(url: String, expires: String) -> Self {
        Self { url, expires }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn expires(&self) -> &str {
        &self.expires
    }
}

#[derive(Debug)]
pub struct SignerError;

impl Display for SignerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "signer error")
    }
}

impl std::error::Error for SignerError {}

#[async_trait::async_trait]
pub trait CloudFileSigner {
    async fn sign(&self, path: &str) -> Result<PresignedUrl, SignerError>;
}
