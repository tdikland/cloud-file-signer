//! An implementation of the [`CloudFileSigner`] trait for Azure Blob Storage.

use std::time::{Duration, SystemTime};

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

use crate::{CloudFileSigner, Permission, PresignedUrl, SignerError};
mod uri;

use self::uri::AzureUri;

/// A signer for Azure Blob Storage.
#[derive(Debug, Clone)]
pub struct AbfsFileSigner {
    storage_account: String,
    client_builder: ClientBuilder,
}

impl AbfsFileSigner {
    /// Create a new signer for Azure Blob Storage.
    pub fn new<A: Into<String>, C: Into<StorageCredentials>>(
        storage_account: A,
        storage_credentials: C,
    ) -> Self {
        let storage_account_name = storage_account.into();
        let client_builder = ClientBuilder::new(storage_account_name.clone(), storage_credentials);
        Self {
            storage_account: storage_account_name,
            client_builder,
        }
    }

    /// Create a new signer for Azure Blob Storage with specified client builder.
    pub fn from_client_builder<A: Into<String>>(
        storage_account: A,
        client_builder: ClientBuilder,
    ) -> Self {
        let storage_account_name = storage_account.into();
        Self {
            storage_account: storage_account_name,
            client_builder,
        }
    }

    /// Return the name of the storage account for which this
    /// signer is configured.
    #[must_use]
    pub fn storage_account(&self) -> &str {
        &self.storage_account
    }

    fn client_builder(&self) -> ClientBuilder {
        self.client_builder.clone()
    }

    async fn sign_read_request(
        &self,
        uri: &AzureUri,
        valid_from: SystemTime,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        if uri.storage_account() != self.storage_account() {
            return Err(SignerError::other_error(
                "Storage account name in URI does not match signer",
            ));
        }

        let start_time = valid_from;
        let end_time = start_time + expiration;
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };

        let blob_client = self
            .client_builder()
            .blob_client(uri.container(), uri.blob());
        let sas_token = blob_client
            .shared_access_signature(permissions, end_time.into())
            .await?;
        let sas_token = sas_token.start(start_time);

        let signed_url = blob_client.generate_signed_blob_url(&sas_token)?;
        Ok(PresignedUrl::new(signed_url, valid_from, expiration))
    }

    async fn sign_write_request(
        &self,
        uri: &AzureUri,
        valid_from: SystemTime,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        if uri.storage_account() != self.storage_account() {
            return Err(SignerError::other_error(
                "Storage account name in URI does not match signer",
            ));
        }

        let start_time = valid_from;
        let end_time = start_time + expiration;
        let permissions = BlobSasPermissions {
            write: true,
            ..Default::default()
        };

        let blob_client = self
            .client_builder()
            .blob_client(uri.container(), uri.blob());
        let sas_token = blob_client
            .shared_access_signature(permissions, end_time.into())
            .await?;
        let sas_token = sas_token.start(start_time);

        let signed_url = blob_client.generate_signed_blob_url(&sas_token)?;
        Ok(PresignedUrl::new(signed_url, valid_from, expiration))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for AbfsFileSigner {
    async fn sign(
        &self,
        path: &str,
        valid_from: SystemTime,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        tracing::info!("signing path: {}", path);
        let azure_uri = path.parse::<AzureUri>()?;
        match permission {
            Permission::Read => Ok(self
                .sign_read_request(&azure_uri, valid_from, expiration)
                .await?),
            Permission::Write => Ok(self
                .sign_write_request(&azure_uri, valid_from, expiration)
                .await?),
        }
    }
}

impl From<azure_storage::Error> for SignerError {
    fn from(e: azure_storage::Error) -> Self {
        Self::other_error(format!("Azure Storage Error: {}", e))
    }
}
