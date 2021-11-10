use ipdisbeacon::bytes::Signature;
use ipdisbeacon::conf::SERVER_PORT_DEFAULT;
use ipdisbeacon::conf::SIGNATURE_DEFAULT;
use std::net::Ipv4Addr;

const SCANNER_PORT_DEFAULT: u16 = 1902;
const SCAN_PERIOD_DEFAULT: f64 = 1.0;
const BROADCAST_ADDR_DEFAULT: Ipv4Addr = Ipv4Addr::BROADCAST; // 255.255.255.255

#[derive(Clone, Debug, PartialEq)]
pub struct ScannerConfig {
    pub port: u16,
    pub scan_period: f64,
    pub broadcast_addr: Ipv4Addr,
    pub target_port: u16,
    pub signature: Signature,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            port: SCANNER_PORT_DEFAULT,
            scan_period: SCAN_PERIOD_DEFAULT,
            broadcast_addr: BROADCAST_ADDR_DEFAULT,
            target_port: SERVER_PORT_DEFAULT,
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
        let conf = ScannerConfig::default();
        assert_eq!(
            conf,
            ScannerConfig {
                port: 1902,
                scan_period: 1.0f64,
                broadcast_addr: Ipv4Addr::new(255, 255, 255, 255),
                target_port: 1901,
                signature: Signature::from("ipdisbeacon")
            }
        );
    }
}
