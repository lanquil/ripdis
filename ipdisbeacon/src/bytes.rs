use serde_json;
use std::fmt;
use tracing::warn;

/// Signature string sent by scanner. Beacon will answer only if matches.
/// Must be shorter than RECV_BUFFER_LENGHT or it will be truncated.
#[derive(Debug)]
pub struct Signature(pub &'static [u8]);

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", safe_format_bytes(self.0))
    }
}

pub type BeaconInfos = serde_json::Value;

/// Message returned to the scanner (JSON formatted).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Answer(pub Vec<u8>);

impl fmt::Display for Answer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.safe_format())
    }
}

impl From<&[u8]> for Answer {
    fn from(bytes: &[u8]) -> Self {
        Self(bytes.into())
    }
}

impl Answer {
    fn safe_format(&self) -> String {
        let res = match serde_json::from_slice(&self.0) {
            Ok(p) => p,
            Err(e) => {
                warn!(?e, "Error deserializing Answer payload.");
                BeaconInfos::String(safe_format_bytes(&self.0))
            }
        };
        res.to_string()
    }
}

fn safe_format_bytes(bytestring: &[u8]) -> String {
    if let Ok(formatted) = std::str::from_utf8(bytestring) {
        formatted.to_string()
    } else {
        format!("INVALID UTF-8: {:?}", bytestring)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_rust() {
        assert_eq!(
            Answer::from(
                serde_json::to_vec(&serde_json::json!(true))
                    .unwrap()
                    .as_slice()
            )
            .safe_format(),
            "true"
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_json() {
        let input = [
            "true",
            "null",
            "4.2",
            r#"{"key string": [1, "two", 3.4, false, null], "2": "another string"}"#,
        ];
        for json in input {
            let data: serde_json::Value = serde_json::from_str(json).unwrap();
            assert_eq!(
                serde_json::from_str::<'_, serde_json::Value>(
                    &Answer::from(json.as_bytes()).safe_format()
                )
                .unwrap(),
                data
            );
        }
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_not_json() {
        let expected = r#""not json""#;
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(
                &Answer::from("not json".as_bytes()).safe_format()
            )
            .unwrap()
            .to_string(),
            expected.to_string()
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_safe_format_answer_not_bytes() {
        let expected = r#""INVALID UTF-8: [31, 32, 255]""#;
        assert_eq!(
            serde_json::from_str::<'_, serde_json::Value>(
                &Answer::from(vec![31u8, 32, 255].as_slice()).safe_format()
            )
            .unwrap()
            .to_string(),
            expected.to_string()
        );
    }
}
