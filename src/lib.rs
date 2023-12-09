pub mod aws;
pub mod azure;
pub mod gcp;

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

pub struct SignerError;

#[async_trait::async_trait]
pub trait CloudFileSigner {
    async fn sign(&self, path: &str) -> Result<PresignedUrl, SignerError>;
}
