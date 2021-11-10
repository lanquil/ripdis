use crate::bytes::Signature;
use std::net::Ipv4Addr;

pub const SERVER_PORT_DEFAULT: u16 = 1901;
pub const SIGNATURE_DEFAULT: &str = "ipdisbeacon"; // must be shorter than RECV_BUFFER_LENGHT
const LISTENING_ADDR_DEFAULT: Ipv4Addr = Ipv4Addr::UNSPECIFIED; // "0.0.0.0"

#[derive(Debug, Clone, PartialEq)]
pub struct BeaconConfig {
    pub port: u16,
    pub listening_addr: Ipv4Addr,
    pub signature: Signature,
}

impl Default for BeaconConfig {
    fn default() -> Self {
        Self {
            port: SERVER_PORT_DEFAULT,
            listening_addr: LISTENING_ADDR_DEFAULT,
            signature: Signature::from(SIGNATURE_DEFAULT),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_conf() {
        let conf = BeaconConfig::default();
        assert_eq!(
            conf,
            BeaconConfig {
                port: 1901,
                listening_addr: Ipv4Addr::new(0, 0, 0, 0),
                signature: Signature::from("ipdisbeacon")
            }
        );
    }
}
