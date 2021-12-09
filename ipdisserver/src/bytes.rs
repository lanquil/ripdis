use bytes::Bytes;

pub fn safe_format_bytes(bytestring: &Bytes) -> String {
    if let Ok(formatted) = std::str::from_utf8(bytestring) {
        formatted.to_string()
    } else {
        let formatted_as_hex_list = format!("{:02X?}", bytestring.as_ref());
        format!("INVALID UTF-8: {}", formatted_as_hex_list)
    }
}
