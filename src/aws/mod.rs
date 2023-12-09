use std::time::Duration;

use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::presigning::PresigningConfigError;
use aws_sdk_s3::Client;

use crate::CloudFileSigner;
use crate::PresignedUrl;
use crate::SignerError;

pub struct S3FileSigner {
    client: Client,
}

impl S3FileSigner {
    fn parse_path(&self, url: &str) -> Result<(String, String), SignerError> {
        let url = url.replace("s3://", "");
        let parts: Vec<&str> = url.split('/').collect();
        if parts.len() < 2 {
            return Err(SignerError {});
        }
        let bucket = parts[0];
        let key = parts[1..].join("/");
        Ok((bucket.to_string(), key.to_string()))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for S3FileSigner {
    async fn sign(&self, path: &str) -> Result<PresignedUrl, SignerError> {
        let (bucket, key) = self.parse_path(path)?;
        let req = self.client.get_object().bucket(bucket).key(key);

        let config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(3600))
            .build()?;
        let res = req.presigned(config).await?;

        Ok(PresignedUrl::new(res.uri().to_string(), String::new()))
    }
}

impl From<PresigningConfigError> for SignerError {
    fn from(_err: PresigningConfigError) -> Self {
        SignerError {}
    }
}

impl From<GetObjectError> for SignerError {
    fn from(_err: GetObjectError) -> Self {
        SignerError {}
    }
}

impl<E, R> From<SdkError<E, R>> for SignerError {
    fn from(_err: SdkError<E, R>) -> Self {
        SignerError {}
    }
}
