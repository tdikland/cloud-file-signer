//! Traits, helpers and type definitions for signing cloud file URLs.
//!
//! `cloud_file_signer` provides a uniform interface for signing URLs.
//! Presigned URLs are useful for granting temporary access to files in
//! cloud storage.
//!
//! # CloudFileSigner
//!
//! The `CloudFileSigner` trait defines a uniform interface for signing
//! URLs. Implementations of `CloudFileSigner` are provided for AWS S3,
//! Azure Blob Storage and Google Cloud Storage.

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::unescaped_backticks)]

use std::{
    fmt::{Display, Formatter},
    time::{Duration, SystemTime},
};

pub mod aws;
pub mod azure;
pub mod gcp;

/// A presigned URL for a file in a (cloud) object store.
///
/// A presigned URL is a URL that grants temporary access to a file in
/// a cloud object store. The URL is signed by the cloud provider and
/// can be used to access the file without further authentication.
///
/// The URL is valid for a limited time, after which it expires and
/// can no longer be used. The URL can also be invalidated by the
/// expiry of the underlying credentials.
///
/// A `PresignedUrl` is typically created by an implementor of the
/// [`CloudFileSigner`] trait.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PresignedUrl {
    url: String,
    valid_from: SystemTime,
    valid_for_duration: Duration,
}

impl PresignedUrl {
    /// Create a new `PresignedUrl`.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    ///
    /// // Create a new presigned URL that is valid for 60 seconds starting now.
    /// // Note that the URL is not actually signed (for brevity).
    /// let presigned_url = PresignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// ```
    pub fn new(
        url: impl Into<String>,
        valid_from: SystemTime,
        valid_for_duration: Duration,
    ) -> Self {
        Self {
            url: url.into(),
            valid_from,
            valid_for_duration,
        }
    }

    /// Return the presigned URL as a string.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(presigned_url.url(), "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key");
    /// ```
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Return the time at which the URL became valid.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert!(presigned_url.valid_from() <= SystemTime::now());
    /// ```
    pub fn valid_from(&self) -> SystemTime {
        self.valid_from
    }

    /// Return the time at which the URL expires.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    ///
    /// let now = SystemTime::now();
    /// let presigned_url = PresignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     now,
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(presigned_url.valid_until(), now + Duration::from_secs(60));
    /// ```
    pub fn valid_until(&self) -> SystemTime {
        self.valid_from + self.valid_for_duration
    }

    /// Return if the URL is expired.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    /// use std::thread::sleep;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(1),
    /// );
    /// assert_eq!(presigned_url.is_expired(), false);
    /// sleep(Duration::from_secs(2));
    /// assert_eq!(presigned_url.is_expired(), true);
    /// ```
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.valid_until()
    }

    /// Return if the URL is still valid.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::PresignedUrl;
    /// use std::thread::sleep;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(1),
    /// );
    /// assert_eq!(presigned_url.is_valid(), true);
    /// sleep(Duration::from_secs(2));
    /// assert_eq!(presigned_url.is_valid(), false);
    /// ```
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

/// Permissions that can be granted to a presigned URL.
#[derive(Debug)]
pub enum Permission {
    /// The URL can be used to read the file.
    Read,
    /// The URL can be used to write to the file.
    Write,
}

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
}

/// An error that occurred while signing a URL.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SignerError {
    kind: SignerErrorKind,
    message: String,
}

impl SignerError {
    fn new(kind: SignerErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    /// Return the kind of error.
    pub fn kind(&self) -> SignerErrorKind {
        self.kind
    }

    /// Return the error message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn uri_parse_error(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::CloudUriParseError, message.into())
    }

    fn permission_not_supported(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::PermissionNotSupported, message.into())
    }

    fn other_error(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::Other, message.into())
    }
}

/// The kind of error that occurred while signing a URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignerErrorKind {
    /// The URI of the object could not be parsed.
    CloudUriParseError,
    /// The requested permission is not supported by the signer.
    PermissionNotSupported,
    /// Some other error occurred.
    Other,
}

impl Display for SignerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SignerError")
    }
}

impl std::error::Error for SignerError {}
