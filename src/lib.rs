//! This create provides abstractions and implementations for creating so
//! called signed URLs. A signed URL is a URL that provides limited permission
//! and time to make a request. Signed URLs contain authentication information
//! in their query string, allowing users without credentials to perform
//! specific actions on a object in (cloud) storage.
//!
//! Here is a quick example of how a signed URL can be created for an object
//! in AWS S3.
//! ```no_run
//! # use cloud_file_signer::{PresignedUrl, SignerError};
//! # fn main() -> Result<(), SignerError> {
//! # async {
//! use std::time::{Duration, SystemTime};
//! use cloud_file_signer::{CloudFileSigner, Permission};
//! use cloud_file_signer::aws::AwsFileSigner;
//!
//! let s3_signer = AwsFileSigner::from_env().await;
//! let signed_url = s3_signer.sign(
//!     "s3://bucket/prefix/key",
//!     SystemTime::now(),
//!     Duration::from_secs(3600),
//!     Permission::Read
//! ).await?;
//! # Ok::<PresignedUrl, SignerError>(signed_url) };
//! # Ok(()) }
//! ```
//!
//! ## When should you use signed URLs?
//! By default objects in (cloud) storage services are private, only
//! authenticated users can access the objects. In some scanarios you might not
//! want to create (cloud) credentials for users to be able to interact with
//! the object storage. An example of such a scenario is an application that
//! allows users to upload images. In this scenario it would be infeasible to
//! create (cloud) credentials for all users, and sharing one set of
//! credentials for all users would violate security best practices.
//! The typical way to address these use cases is to provide a signed URL to
//! the user, which gives the user read or write access to a specific object
//! for a limited amount of time.
//!
//! # `CloudFileSigner`
//! The `CloudFileSigner` trait defines a uniform interface for signing
//! URLs. Implementations of `CloudFileSigner` are provided for AWS S3,
//! Azure Blob Storage and Google Cloud Storage.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::unescaped_backticks)]

use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

pub mod aws;
pub mod azure;
pub mod gcp;

mod error;
mod permissions;
mod presigned_url;

pub use error::{SignerError, SignerErrorKind};
pub use permissions::Permission;
pub use presigned_url::PresignedUrl;

/// A trait for signing URLs for files in a cloud object store.
#[async_trait::async_trait]
pub trait CloudFileSigner: Send + Sync {
    /// Sign a URL for a file in a cloud object store. The URL is valid
    /// for the specified duration and grants the specified
    /// permission.
    async fn sign(
        &self,
        path: &str,
        valid_from: SystemTime,
        expires_in: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError>;

    /// Sign a URL for a file in a cloud object store. The URL is valid
    /// for the specified duration and grants read permission.
    async fn sign_read_only_starting_now(
        &self,
        path: &str,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        self.sign(path, SystemTime::now(), expiration, Permission::Read)
            .await
    }

    /// Sign a URL for a file in a cloud object store. The URL is valid
    /// for the specified duration and grants write permission.
    async fn sign_write_only_starting_now(
        &self,
        path: &str,
        expiration: Duration,
    ) -> Result<PresignedUrl, SignerError> {
        self.sign(path, SystemTime::now(), expiration, Permission::Write)
            .await
    }
}

#[async_trait::async_trait]
impl<S: CloudFileSigner + ?Sized> CloudFileSigner for Box<S> {
    async fn sign(
        &self,
        path: &str,
        valid_from: SystemTime,
        expires_in: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        (**self)
            .sign(path, valid_from, expires_in, permission)
            .await
    }
}

#[async_trait::async_trait]
impl<S: CloudFileSigner + ?Sized> CloudFileSigner for Arc<S> {
    async fn sign(
        &self,
        path: &str,
        valid_from: SystemTime,
        expires_in: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError> {
        (**self)
            .sign(path, valid_from, expires_in, permission)
            .await
    }
}
