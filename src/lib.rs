//! Traits, helpers and type definitions for signing cloud file URLs.
//!
//! `cloud_file_signer` provides a uniform interface for signing URLs.
//! Presigned URLs are useful for granting temporary access to files in
//! cloud storage.
//!
//! # `CloudFileSigner`
//!
//! The `CloudFileSigner` trait defines a uniform interface for signing
//! URLs. Implementations of `CloudFileSigner` are provided for AWS S3,
//! Azure Blob Storage and Google Cloud Storage.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::unescaped_backticks)]

use std::time::{Duration, SystemTime};

pub mod aws;
pub mod azure;
pub mod gcp;

pub mod error;
pub mod permissions;
pub mod presigned_url;

use error::SignerError;
use permissions::Permission;
use presigned_url::PresignedUrl;

/// A trait for signing URLs for files in a cloud object store.
#[async_trait::async_trait]
pub trait CloudFileSigner {
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
