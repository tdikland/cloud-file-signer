//! Types for representing permissions that can be granted to a presigned URL.

use std::fmt::{Display, Formatter};

/// Permissions that can be granted to a presigned URL.
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
