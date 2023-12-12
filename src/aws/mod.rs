use std::str::FromStr;
use std::time::Duration;

use aws_config::SdkConfig;
use aws_sdk_s3::error::SdkError;
use aws_sdk_s3::operation::get_object::GetObjectError;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::presigning::PresigningConfigError;
use aws_sdk_s3::Client;

use http::Uri;

use crate::CloudFileSigner;
use crate::PresignedUrl;
use crate::SignerError;

#[derive(Debug, Clone)]
pub struct S3FileSigner {
    client: Client,
}

impl S3FileSigner {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn from_config(config: &SdkConfig) -> Self {
        let client = Client::new(config);
        Self { client }
    }

    pub async fn from_env() -> Self {
        let config = aws_config::load_from_env().await;
        let client = Client::new(&config);
        Self { client }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct S3Uri {
    bucket: String,
    region: Option<String>,
    key: String,
}

impl S3Uri {
    pub fn new(bucket: impl Into<String>, region: Option<String>, key: impl Into<String>) -> Self {
        Self {
            bucket: bucket.into(),
            region,
            key: key.into(),
        }
    }

    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn to_url(&self) -> String {
        format!(
            "https://{0}.s3.amazonaws.com/{1}",
            self.bucket(),
            self.key()
        )
    }
}

impl FromStr for S3Uri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri: Uri = s.parse().map_err(|_| SignerError {})?;

        // TODO: there are many naming schemes
        if let Some(s) = uri.scheme_str() {
            match s {
                "https" => (),
                _ => panic!("invalid scheme"),
            }
        } else {
            return Err(SignerError {});
        };

        let bucket = if let Some(host) = uri.host() {
            host.split(".").next().unwrap()
        } else {
            return Err(SignerError {});
        };

        let key = uri.path().strip_prefix("/").unwrap();

        Ok(S3Uri::new(bucket.to_string(), None, key.to_string()))
    }
}

impl S3FileSigner {
    // fn parse_path(&self, url: &str) -> Result<(String, String), SignerError> {
    //     let url = url.replace("s3://", "");
    //     let parts: Vec<&str> = url.split('/').collect();
    //     if parts.len() < 2 {
    //         return Err(SignerError {});
    //     }
    //     let bucket = parts[0];
    //     let key = parts[1..].join("/");
    //     Ok((bucket.to_string(), key.to_string()))
    // }
}

#[async_trait::async_trait]
impl CloudFileSigner for S3FileSigner {
    async fn sign(&self, path: &str) -> Result<PresignedUrl, SignerError> {
        // let (bucket, key) = self.parse_path(path)?;
        let s3_uri = path.parse::<S3Uri>()?;

        let req = self
            .client
            .get_object()
            .bucket(s3_uri.bucket())
            .key(s3_uri.key());

        let config = PresigningConfig::builder()
            .expires_in(Duration::from_secs(3600))
            .build()?;
        let res = req.presigned(config).await?;

        Ok(PresignedUrl::new(res.uri().to_string(), String::new()))
    }
}

impl From<PresigningConfigError> for SignerError {
    fn from(_err: PresigningConfigError) -> Self {
        SignerError {}
    }
}

impl From<GetObjectError> for SignerError {
    fn from(_err: GetObjectError) -> Self {
        SignerError {}
    }
}

impl<E, R> From<SdkError<E, R>> for SignerError {
    fn from(_err: SdkError<E, R>) -> Self {
        SignerError {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_s3_uri() {
        let s3_uri = "https://foo.s3.us-east-1.amazonaws.com/bar";
        let parsed_s3_uri = s3_uri.parse::<S3Uri>().unwrap();
        assert_eq!(parsed_s3_uri.bucket(), "foo");
        assert_eq!(parsed_s3_uri.key(), "bar");
    }
}
