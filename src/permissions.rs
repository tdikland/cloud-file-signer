//! Types for representing permissions that can be granted to a presigned URL.

/// Permissions that can be granted to a presigned URL.
#[derive(Debug)]
pub enum Permission {
    /// The URL can be used to read the file.
    Read,
    /// The URL can be used to write to the file.
    Write,
}
