use http::Uri;
use std::str::FromStr;

use crate::error::SignerError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct S3Uri {
    bucket: String,
    key: String,
    region: Option<String>,
}

impl S3Uri {
    pub fn new(bucket: String, key: String, region: Option<String>) -> Self {
        Self {
            bucket,
            key,
            region,
        }
    }

    fn from_s3_uri(uri: &Uri) -> Result<Self, SignerError> {
        let bucket = uri
            .host()
            .ok_or(SignerError::uri_parse_error("Invalid URI: missing bucket"))?;

        let key = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::uri_parse_error("Invalid URI: bad key"))?;

        Ok(Self::new(bucket.to_string(), key.to_string(), None))
    }

    fn from_url(uri: &Uri) -> Result<Self, SignerError> {
        let host = uri
            .host()
            .ok_or(SignerError::uri_parse_error("Invalid URI"))?;

        let re = regex::Regex::new("^(.+\\.)?s3[.-]([a-z0-9-]+)\\.")
            .map_err(|_| SignerError::other_error("regex compilation failed"))?;
        let cap = re.captures(host).ok_or(SignerError::uri_parse_error(
            "Invalid URI. Hostname does not appear to be a valid S3 endpoint",
        ))?;
        let _region = cap.get(2).map(|m| m.as_str());
        let prefix = cap.get(1).map(|m| m.as_str());

        if let Some(p) = prefix {
            Self::parse_virtual_hosted_style_url(uri, p)
        } else {
            Self::parse_path_style_url(uri.clone())
        }
    }

    fn parse_virtual_hosted_style_url(uri: &Uri, bucket: &str) -> Result<Self, SignerError> {
        let key = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::uri_parse_error("Invalid URI: bad path"))?;

        Ok(Self {
            bucket: bucket.strip_suffix('.').unwrap().to_string(),
            key: key.to_string(),
            region: None,
        })
    }

    fn parse_path_style_url(uri: Uri) -> Result<Self, SignerError> {
        let path = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::uri_parse_error(
                "Invalid URI: missing bucket and key",
            ))?;

        let (bucket, key) = path
            .split_once('/')
            .ok_or(SignerError::uri_parse_error("Invalid URI: missing key"))?;

        Ok(Self {
            bucket: bucket.to_string(),
            key: key.to_string(),
            region: None,
        })
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &str {
        &self.key
    }
}

impl FromStr for S3Uri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri: Uri = s
            .parse()
            .map_err(|e| SignerError::uri_parse_error(format!("Invalid URI: {e}")))?;

        match uri.scheme_str() {
            Some("s3" | "s3a" | "s3n") => Ok(Self::from_s3_uri(&uri)?),
            Some("http" | "https") => Ok(Self::from_url(&uri)?),
            _ => Err(SignerError::uri_parse_error("Invalid URI scheme")),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_s3_scheme() {
        let uri = "s3://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3a_scheme() {
        let uri = "s3a://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3n_scheme() {
        let uri = "s3n://bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_s3_nested_key() {
        let uri = "s3://bucket/key/nested";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key/nested");
    }

    #[test]
    fn parse_http_scheme() {
        let uri = "http://bucket.s3.us-east-1.amazonaws.com/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_https_scheme() {
        let uri = "https://bucket.s3.us-east-1.amazonaws.com/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_virtual_hosted_global() {
        let uri = "https://bucket.s3.amazonaws.com/key/nested";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key/nested");
    }

    #[test]
    fn parse_virtual_hosted_regional() {
        let uri = "https://bucket.s3.us-east-1.amazonaws.com/key/nested";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key/nested");
    }

    #[test]
    fn parse_path_style_global() {
        let uri = "https://s3.amazonaws.com/bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_path_style_regional() {
        let uri = "https://s3.us-east-1.amazonaws.com/bucket/key";
        let s3_uri = S3Uri::from_str(uri).unwrap();
        assert_eq!(s3_uri.bucket(), "bucket");
        assert_eq!(s3_uri.key(), "key");
    }

    #[test]
    fn parse_invalid_scheme() {
        let uri = "abfss://bucket.s3.us-east-1.amazonaws.com/key";
        let s3_uri = S3Uri::from_str(uri);
        assert!(s3_uri.is_err());
    }
}
