use std::fmt;

/// Signature string sent by scanner. Beacon will answer only if matches.
/// Must be shorter than RECV_BUFFER_LENGHT or it will be truncated.
#[derive(Debug)]
pub struct Signature(pub &'static [u8]);

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_bytes(self.0))
    }
}

/// Message returned to the scanner. JSON formatted.
#[derive(Debug, Default, Clone)]
pub struct Answer(pub Vec<u8>);

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_bytes(&self.0))
    }
}

impl From<&[u8]> for Answer {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.into())
    }
}

fn format_bytes(bytestring: &[u8]) -> String {
    if let Ok(formatted) = std::str::from_utf8(bytestring) {
        format!("b\"{}\"", formatted)
    } else {
        format!("{:?}", bytestring)
    }
}
