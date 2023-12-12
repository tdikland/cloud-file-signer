use aws_sdk_s3::Client;
use azure_core::error::{ErrorKind, ResultExt};
use azure_storage::{prelude::*, shared_access_signature};
use azure_storage_blobs::prelude::*;

use crate::{CloudFileSigner, PresignedUrl, SignerError};

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
}

use azure_storage::clients::shared_access_signature;

#[async_trait::async_trait]
impl CloudFileSigner for AbfsFileSigner {
    async fn sign(&self, path: &str) -> Result<PresignedUrl, SignerError> {
        let (container, blob) = path.split_once("/").unwrap();

        let mut permissions = BlobSasPermissions::default();
        permissions.read = true;
        let dt = time::OffsetDateTime::now_utc() + time::Duration::hours(3);

        let blob_client = self.client_builder.clone().blob_client(container, blob);
        let sas_token = blob_client
            .shared_access_signature(permissions, dt)
            .await
            .unwrap();
        let sas_token = sas_token.start(time::OffsetDateTime::now_utc());

        // INCORRECT - gives access at a higher level than the resource
        // let storage_credentials = StorageCredentials::emulator();
        // let sas_token = shared_access_signature(
        //     &storage_credentials,
        //     AccountSasResource::Blob,
        //     AccountSasResourceType::Object,
        //     time::OffsetDateTime::now_utc() + time::Duration::hours(3),
        //     AccountSasPermissions {
        //         read: true,
        //         ..Default::default()
        //     },
        // )
        // .await
        // .unwrap();

        let url = blob_client.generate_signed_blob_url(&sas_token).unwrap();
        println!("URL: {}", url);

        Ok(PresignedUrl::new(url.into(), String::new()))
    }
}
// let file_name = "azure_sdk_for_rust_stream_test.txt";

// // First we retrieve the account name and access key from environment variables.
// let account = std::env::var("STORAGE_ACCOUNT").expect("missing STORAGE_ACCOUNT");
// let access_key = std::env::var("STORAGE_ACCESS_KEY").expect("missing STORAGE_ACCOUNT_KEY");
// let container = std::env::var("STORAGE_CONTAINER").expect("missing STORAGE_CONTAINER");
// let blob_name = std::env::var("STORAGE_BLOB_NAME").expect("missing STORAGE_BLOB_NAME");

// let storage_credentials = StorageCredentials::access_key(account.clone(), access_key);
// let blob_client = ClientBuilder::new(account, storage_credentials).blob_client(&container, blob_name);

// blob_client.put_block_blob("hello world").content_type("text/plain").await?;

// let mut result: Vec<u8> = vec![];

// // The stream is composed of individual calls to the get blob endpoint
// let mut stream = blob_client.get().into_stream();
// while let Some(value) = stream.next().await {
//     let mut body = value?.data;
//     // For each response, we stream the body instead of collecting it all
//     // into one large allocation.
//     while let Some(value) = body.next().await {
//         let value = value?;
//         result.extend(&value);
//     }
// }

// println!("result: {:?}", result);

// Ok(())
