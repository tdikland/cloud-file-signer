//! An implementation of the [`CloudFileSigner`] trait for Google Cloud Storage.

use std::collections::HashMap;
use std::time::Duration;
use std::time::SystemTime;

use chrono::DateTime;
use chrono::SecondsFormat;
use chrono::Utc;
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
    #[must_use]
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
        valid_from: SystemTime,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        let mut query_params = HashMap::new();
        query_params.insert(
            String::from("X-Goog-Date"),
            vec![DateTime::<Utc>::from(valid_from).to_rfc3339_opts(SecondsFormat::Secs, true)],
        );

        let opts = SignedURLOptions {
            expires: expiration,
            method: SignedURLMethod::GET,
            query_parameters: query_params,
            ..Default::default()
        };

        let signed_url = self
            .client
            .signed_url(uri.bucket(), uri.key(), None, None, opts)
            .await
            .map_err(|e| SignerError::other_error(e.to_string()))?;
        Ok(PresignedUrl::new(signed_url, valid_from, expiration))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for GcpFileSigner {
    async fn sign(
        &self,
        path: &str,
        valid_from: SystemTime,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        let uri = path.parse::<GcpUri>()?;
        match permission {
            Permission::Read => self.sign_read_request(&uri, valid_from, expiration).await,
            Permission::Write => Err(SignerError::permission_not_supported(
                "GCP does not support write permissions",
            )),
        }
    }
}
