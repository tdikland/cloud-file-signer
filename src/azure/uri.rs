use std::str::FromStr;

use http::Uri;

use crate::SignerError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AzureUri {
    storage_account: String,
    container: String,
    blob: String,
}

impl AzureUri {
    pub fn new(storage_account: String, container: String, blob: String) -> Self {
        Self {
            storage_account,
            container,
            blob,
        }
    }

    pub fn storage_account(&self) -> &str {
        &self.storage_account
    }

    pub fn container(&self) -> &str {
        &self.container
    }

    pub fn blob(&self) -> &str {
        &self.blob
    }

    pub fn parse_abfss_uri(uri: &Uri) -> Result<Self, SignerError> {
        let storage_account = uri
            .host()
            .and_then(|d| d.split_once('.'))
            .map(|m| m.0)
            .ok_or(SignerError::uri_parse_error(
                "Invalid URI: couldn't extract storage account name. Format the URI as `abfss://<container>@<storage_account>.dfs.core.windows.net/path/to/blob`",
            ))?;
        let container = uri
            .authority()
            .and_then(|a| a.as_str().split_once('@'))
            .map(|m| m.0)
            .ok_or(SignerError::uri_parse_error(
                "Invalid URI: couldn't extract container name. Format the URI as `abfss://<container>@<storage_account>.dfs.core.windows.net/path/to/blob`",
            ))?;
        let blob = uri
            .path()
            .strip_prefix('/')
            .ok_or(SignerError::uri_parse_error(
                "Invalid URI: couldn't extract blob name. Format the URI as `abfss://<container>@<storage_account>.dfs.core.windows.net/path/to/blob`",
            ))?;

        Ok(Self::new(
            storage_account.into(),
            container.into(),
            blob.into(),
        ))
    }
}

impl FromStr for AzureUri {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uri = Uri::from_str(s).map_err(|_| {
            SignerError::uri_parse_error("Invalid URI. Could not parse URI string into URI")
        })?;

        match uri.scheme_str() {
            Some("abfss" | "abfs") => Self::parse_abfss_uri(&uri),
            None => Err(SignerError::uri_parse_error(
                format!("Invalid URI: missing scheme. The URI should start with `abfs` or `abfss`. Received URI: `{s}`."))
            ),
            Some(unsupported_scheme) => Err(SignerError::uri_parse_error(
                format!("Unsupported URI scheme. Supported schemas are `abfss` and `abfs`. Received scheme: `{unsupported_scheme}`."),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::SignerErrorKind;

    use super::*;

    #[test]
    fn parse_abfss_scheme() {
        let uri = "abfss://mycontainer@mystorageaccount.dfs.core.windows.net/myblob";
        let azure_uri = AzureUri::from_str(uri).unwrap();
        assert_eq!(azure_uri.storage_account(), "mystorageaccount");
        assert_eq!(azure_uri.container(), "mycontainer");
        assert_eq!(azure_uri.blob(), "myblob");
    }

    #[test]
    fn parse_abfs_scheme() {
        let uri = "abfs://mycontainer@mystorageaccount.dfs.core.windows.net/myblob";
        let azure_uri = AzureUri::from_str(uri).unwrap();
        assert_eq!(azure_uri.storage_account(), "mystorageaccount");
        assert_eq!(azure_uri.container(), "mycontainer");
        assert_eq!(azure_uri.blob(), "myblob");
    }

    #[test]
    fn parse_unsupported_scheme() {
        let uri = "invalid://mystorageaccount.dfs.core.windows.net/mycontainer/myblob";
        let uri_err = AzureUri::from_str(uri).unwrap_err();
        assert_eq!(uri_err.kind(), SignerErrorKind::CloudUriParseError);
        assert_eq!(uri_err.message(), "Unsupported URI scheme. Supported schemas are `abfss` and `abfs`. Received scheme: `invalid`.")
    }
}
