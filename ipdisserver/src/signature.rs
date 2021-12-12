use crate::bytes::safe_format_bytes;
use bytes::Bytes;
use std::fmt;

/// Signature string sent by scanner. Beacon will answer only if matches.
/// Must be shorter than RECV_BUFFER_LENGHT or it will be truncated.
#[derive(Debug, Clone, PartialEq)]
pub struct Signature(pub Bytes);

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", safe_format_bytes(&self.0))
    }
}

impl From<&str> for Signature {
    fn from(string: &str) -> Self {
        Self(Bytes::copy_from_slice(
            &string.bytes().into_iter().collect::<Vec<u8>>(),
        ))
    }
}

impl From<&[u8]> for Signature {
    fn from(bytes: &[u8]) -> Self {
        Self(Bytes::copy_from_slice(bytes))
    }
}
