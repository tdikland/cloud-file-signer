use std::str::FromStr;

use http::Uri;

use crate::SignerError;

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
            .strip_prefix("/")
            .ok_or(SignerError::uri_parse_error("Invalid URI: bad key"))?;

        Ok(Self::new(bucket.to_string(), key.to_string()))
    }
}

impl FromStr for GcpUri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = Uri::from_str(s).map_err(|e| SignerError::uri_parse_error(e.to_string()))?;
        Self::parse_gs_uri(uri)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_gs_uri() {
        let uri = "gs://bucket/key".parse::<GcpUri>().unwrap();
        assert_eq!(uri.bucket(), "bucket");
        assert_eq!(uri.key(), "key");
    }
}
