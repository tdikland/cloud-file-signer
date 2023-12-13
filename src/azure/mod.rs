//! An implementation of the [`CloudFileSigner`] trait for Azure Blob Storage.

use std::time::{Duration, SystemTime};

use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;

use crate::{CloudFileSigner, Permission, PresignedUrl, SignerError};
mod uri;

use self::uri::AzureUri;

pub struct AbfsFileSigner {
    client_builder: ClientBuilder,
    storage_account_name: String,
}

impl AbfsFileSigner {
    // fn new(cb: ClientBuilder) -> Self {
    //     Self {
    //         storage_account_name: todo!(),
    //     }
    // }

    // TODO: remove and refactor
    pub fn emulator() -> Self {
        let cb = ClientBuilder::emulator();
        let account_name = "storageaccount1";
        Self {
            client_builder: cb,
            storage_account_name: account_name.into(),
        }
    }

    fn storage_account_name(&self) -> &str {
        &self.storage_account_name
    }

    async fn sign_read_request(
        &self,
        uri: &AzureUri,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        if uri.storage_account() != self.storage_account_name() {
            return Err(SignerError::other_error(
                "Storage account name in URI does not match signer",
            ));
        }

        let valid_from = SystemTime::now();
        let permissions = BlobSasPermissions {
            read: true,
            ..Default::default()
        };
        let expiry = time::OffsetDateTime::now_utc() + expiration;

        let blob_client = self
            .client_builder
            .clone()
            .blob_client(uri.container(), uri.blob());
        let sas_token = blob_client
            .shared_access_signature(permissions, expiry)
            .await
            .unwrap();
        let sas_token = sas_token.start(time::OffsetDateTime::now_utc());

        let url = blob_client.generate_signed_blob_url(&sas_token).unwrap();
        Ok(PresignedUrl::new(url, valid_from, expiration))
    }
}

#[async_trait::async_trait]
impl CloudFileSigner for AbfsFileSigner {
    async fn sign(
        &self,
        path: &str,
        _valid_from: SystemTime,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        tracing::info!("signing path: {}", path);
        let azure_uri = path.parse::<AzureUri>()?;
        match permission {
            Permission::Read => Ok(self.sign_read_request(&azure_uri, expiration).await?),
            _ => Err(SignerError::permission_not_supported(format!(
                "permission {:?} not supported",
                permission
            ))),
        }
    }
}
