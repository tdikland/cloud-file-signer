//! An implementation of the [`CloudFileSigner`] trait for Amazon S3.

use std::time::Duration;
use std::time::SystemTime;

use aws_config::SdkConfig;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::presigning::PresigningConfigError;
use aws_sdk_s3::Client;

use crate::error::SignerError;
use crate::permissions::Permission;
use crate::presigned_url::PresignedUrl;
use crate::CloudFileSigner;

mod uri;

/// A signer for Amazon S3.
#[derive(Debug, Clone)]
pub struct S3FileSigner {
    client: Client,
}

impl S3FileSigner {
    /// Create a new signer for Amazon S3.
    #[must_use] pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new signer for Amazon S3 from a [`SdkConfig`].
    pub async fn from_config(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        Self { client }
    }

    /// Create a new signer for Amazon S3 from the environment.
    pub async fn from_env() -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client }
    }
}

impl S3FileSigner {
    async fn sign_get_request(
        &self,
        uri: &uri::S3Uri,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        let valid_from = SystemTime::now();

        let cfg = PresigningConfig::builder().expires_in(expiration).build()?;
        let presigned_request = self
            .client
            .get_object()
            .bucket(uri.bucket())
            .key(uri.key())
            .presigned(cfg)
            .await?;

        Ok(PresignedUrl::new(
            presigned_request.uri().to_string(),
            valid_from,
            expiration,
        ))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for S3FileSigner {
    async fn sign(
        &self,
        path: &str,
        _valid_from: SystemTime,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        let s3_uri = path.parse::<uri::S3Uri>()?;
        match permission {
            Permission::Read => Ok(self.sign_get_request(&s3_uri, expiration).await?),
            _ => Err(SignerError::permission_not_supported(format!(
                "permission {permission:?} not supported"
            ))),
        }
    }
}

impl From<PresigningConfigError> for SignerError {
    fn from(e: PresigningConfigError) -> Self {
        SignerError::other_error(format!("Other error: {e}"))
    }
}

impl From<GetObjectError> for SignerError {
    fn from(e: GetObjectError) -> Self {
        SignerError::other_error(format!("Other error: {e}"))
    }
}

impl<E, R> From<SdkError<E, R>> for SignerError {
    fn from(e: SdkError<E, R>) -> Self {
        SignerError::other_error(format!("Other error: {e}"))
    }
}
