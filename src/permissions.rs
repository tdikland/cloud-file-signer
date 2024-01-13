//! Types for representing permissions that can be granted to a presigned URL.

use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use crate::error::SignerError;

/// Permissions that can be granted to a presigned URL.
///
/// When an URL is signed, the signature also includes the
/// actions that a use can do with the given URL.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Permission {
    /// The URL can be used to read the file.
    Read,
    /// The URL can be used to write to the file.
    Write,
}

impl Display for Permission {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Permission::Read => write!(f, "ReadOnly"),
            Permission::Write => write!(f, "WriteOnly"),
        }
    }
}

impl FromStr for Permission {
    type Err = SignerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "r" | "read" | "readonly" => Ok(Self::Read),
            "w" | "write" | "writeonly" => Ok(Self::Write),
            u => Err(SignerError::permission_not_supported(format!(
                "`{u}` is not recognized as a valid permission for a presigned url."
            ))),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::error::SignerErrorKind;

    use super::*;

    #[test]
    fn display_permission() {
        let read_only = Permission::Read;
        assert_eq!(&read_only.to_string(), "ReadOnly");

        let write_only = Permission::Write;
        assert_eq!(&write_only.to_string(), "WriteOnly");
    }

    #[test]
    fn parse_permission() {
        let parsed_read = ["r", "read", "readonly"]
            .into_iter()
            .map(|s| s.parse::<Permission>().unwrap());
        assert!(parsed_read.into_iter().all(|p| p == Permission::Read));

        let parsed_write = ["w", "write", "writeonly"]
            .into_iter()
            .map(|s| s.parse::<Permission>().unwrap());
        assert!(parsed_write.into_iter().all(|p| p == Permission::Write));

        let failed_parse = "unknown permission".parse::<Permission>().unwrap_err();
        assert_eq!(failed_parse.kind(), SignerErrorKind::PermissionNotSupported);
        assert_eq!(
            failed_parse.message(),
            "`unknown permission` is not recognized as a valid permission for a presigned url."
        )
    }
}
