use std::str::FromStr;

use http::Uri;

use crate::SignerError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct GcpUri {
    bucket: String,
    key: String,
}

impl GcpUri {
    pub fn new(bucket: impl Into<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            key: key.into(),
        }
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    fn parse_gs_uri(uri: Uri) -> Result<Self, SignerError> {
        let bucket = uri
            .host()
            .ok_or(SignerError::uri_parse_error("Invalid URI: missing bucket"))?;

        let key = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::uri_parse_error("Invalid URI: bad key"))?;

        Ok(Self::new(bucket.to_string(), key.to_string()))
    }
}

impl FromStr for GcpUri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = Uri::from_str(s).map_err(|e| SignerError::uri_parse_error(e.to_string()))?;
        match uri.scheme_str() {
            Some("gs") => Self::parse_gs_uri(uri),
            None => Err(SignerError::uri_parse_error(
                format!("Invalid URI: missing scheme. The URI should start with `gs`. Received URI: `{s}`."))
            ),
            Some(unsupported_scheme) => Err(SignerError::uri_parse_error(
                format!("Unsupported URI scheme. Supported schemas is `gs`. Received scheme: `{unsupported_scheme}`."),
            )),}
    }
}

#[cfg(test)]
mod test {
    use crate::error::SignerErrorKind;

    use super::*;

    #[test]
    fn parse_gs_uri() {
        let uri = "gs://bucket/key";
        let gcp_uri = uri.parse::<GcpUri>().unwrap();
        assert_eq!(gcp_uri.bucket(), "bucket");
        assert_eq!(gcp_uri.key(), "key");
    }

    #[test]
    fn parse_unsupported_scheme() {
        let uri = "invalid://bucket/key";
        let uri_err = uri.parse::<GcpUri>().unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::CloudUriParseError);
        assert_eq!(
            uri_err.message(),
            "Unsupported URI scheme. Supported schemas is `gs`. Received scheme: `invalid`."
        )
    }
}
