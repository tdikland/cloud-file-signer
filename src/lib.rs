use std::{
    fmt::{Display, Formatter},
    time::Duration,
};

pub mod aws;
pub mod azure;
pub mod gcp;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PresignedUrl {
    url: String,
    expires: String,
}

impl PresignedUrl {
    fn new(url: String, expires: String) -> Self {
        Self { url, expires }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn expires(&self) -> &str {
        &self.expires
    }
}

#[derive(Debug)]
pub struct SignerError {
    kind: SignerErrorKind,
    message: String,
}

impl SignerError {
    pub fn new(kind: SignerErrorKind, message: String) -> Self {
        Self { kind, message }
    }

    pub fn kind(&self) -> &SignerErrorKind {
        &self.kind
    }

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

#[derive(Debug)]
pub enum SignerErrorKind {
    CloudUriParseError,
    PermissionNotSupported,
    Other,
}

impl Display for SignerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "SignerError")
    }
}

impl std::error::Error for SignerError {}

#[derive(Debug)]
pub enum Permission {
    Read,
    Write,
}

#[async_trait::async_trait]
pub trait CloudFileSigner {
    async fn sign(
        &self,
        path: &str,
        expiration: Duration,
        permission: Permission,
    ) -> Result<PresignedUrl, SignerError>;
}
