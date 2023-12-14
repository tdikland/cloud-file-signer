//! A presigned URL for a file in a (cloud) object store.

use std::{
    fmt::{Display, Formatter},
    time::{Duration, SystemTime},
};

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
/// `CloudFileSigner` trait.
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
    /// use cloud_file_signer::presigned_url::PresignedUrl;
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
    /// use cloud_file_signer::presigned_url::PresignedUrl;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///     "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(presigned_url.url(), "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key");
    /// ```
    #[must_use]
    pub fn url(&self) -> &str {
        &self.url
    }

    /// Return the time at which the URL became valid.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::presigned_url::PresignedUrl;
    ///
    /// let presigned_url = PresignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     SystemTime::now(),
    ///     Duration::from_secs(60),
    /// );
    /// assert!(presigned_url.valid_from() <= SystemTime::now());
    /// ```
    #[must_use]
    pub fn valid_from(&self) -> SystemTime {
        self.valid_from
    }

    /// Return the time at which the URL expires.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::presigned_url::PresignedUrl;
    ///
    /// let now = SystemTime::now();
    /// let presigned_url = PresignedUrl::new(
    ///    "https://my_bucket.s3.eu-west-1.amazonaws.com/my_key",
    ///     now,
    ///     Duration::from_secs(60),
    /// );
    /// assert_eq!(presigned_url.valid_until(), now + Duration::from_secs(60));
    /// ```
    #[must_use]
    pub fn valid_until(&self) -> SystemTime {
        self.valid_from + self.valid_for_duration
    }

    /// Return if the URL is expired.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::presigned_url::PresignedUrl;
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
    #[must_use]
    pub fn is_expired(&self) -> bool {
        SystemTime::now() > self.valid_until()
    }

    /// Return if the URL is still valid.
    ///
    /// # Example
    /// ```rust
    /// use std::time::{Duration, SystemTime};
    /// use cloud_file_signer::presigned_url::PresignedUrl;
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
    #[must_use]
    pub fn is_valid(&self) -> bool {
        !self.is_expired()
    }
}

impl Display for PresignedUrl {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.url)
    }
}

impl AsRef<str> for PresignedUrl {
    fn as_ref(&self) -> &str {
        &self.url
    }
}
