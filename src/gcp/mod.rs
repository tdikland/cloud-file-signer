//! An implementation of the [`CloudFileSigner`] trait for Google Cloud Storage.

use std::time::Duration;
use std::time::SystemTime;

use google_cloud_storage::client::Client;
use google_cloud_storage::client::ClientConfig;
use google_cloud_storage::sign::SignedURLMethod;
use google_cloud_storage::sign::SignedURLOptions;

use crate::CloudFileSigner;
use crate::Permission;
use crate::PresignedUrl;
use crate::SignerError;

use self::uri::GcpUri;

mod uri;

/// A signer for Google Cloud Storage.
pub struct GcpFileSigner {
    client: Client,
}

impl GcpFileSigner {
    /// Create a new signer for Google Cloud Storage.
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Create a new signer for Google Cloud Storage using environment variables.
    pub async fn from_env() -> Self {
        let client_config = ClientConfig::default().with_auth().await.unwrap();
        let client = Client::new(client_config);
        Self { client }
    }

    async fn sign_read_request(
        &self,
        uri: &GcpUri,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        let valid_from = SystemTime::now();

        let opts = SignedURLOptions {
            expires: expiration,
            method: SignedURLMethod::GET,
            ..Default::default()
        };

        let url = self
            .client
            .signed_url(uri.bucket(), uri.key(), None, None, opts)
            .await
            .map_err(|e| SignerError::other_error(e.to_string()))?;
        Ok(PresignedUrl::new(url, valid_from, expiration))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for GcpFileSigner {
    async fn sign(
        &self,
        path: &str,
        _valid_from: SystemTime,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        let uri = path.parse::<GcpUri>()?;
        match permission {
            Permission::Read => self.sign_read_request(&uri, expiration).await,
            Permission::Write => Err(SignerError::permission_not_supported(
                "GCP does not support write permissions",
            )),
        }
    }
}
