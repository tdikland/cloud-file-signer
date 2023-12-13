//! Errors that can occur while signing a URL.

use std::fmt::{Display, Formatter};

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

    /// Create a new CloudUriParseError.
    pub fn uri_parse_error(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::CloudUriParseError, message.into())
    }

    /// Create a new PermissionNotSupported error.
    pub fn permission_not_supported(message: impl Into<String>) -> Self {
        Self::new(SignerErrorKind::PermissionNotSupported, message.into())
    }

    /// Create a new Other error.
    pub fn other_error(message: impl Into<String>) -> Self {
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
