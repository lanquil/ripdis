use ipdisserver::conf::SERVER_PORT_DEFAULT;
use ipdisserver::conf::SIGNATURE_DEFAULT;
use ipdisserver::signature::Signature;
use std::net::Ipv4Addr;

const SCANNER_PORT_DEFAULT: u16 = 1902;
const SCAN_PERIOD_DEFAULT: f64 = 1.0;
const BROADCAST_ADDR_DEFAULT: Ipv4Addr = Ipv4Addr::BROADCAST; // 255.255.255.255
const EXTRA_SIGNATURE_DEFAULT: &str = "pang-supremacy-maritime-revoke-afterglow"; // compatibility with original ipdiscan

#[derive(Clone, Debug, PartialEq)]
pub struct ScannerConfig {
    pub port: u16,
    pub scan_period: f64,
    pub broadcast_addr: Ipv4Addr,
    pub target_port: u16,
    pub signatures: Vec<Signature>,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            port: SCANNER_PORT_DEFAULT,
            scan_period: SCAN_PERIOD_DEFAULT,
            broadcast_addr: BROADCAST_ADDR_DEFAULT,
            target_port: SERVER_PORT_DEFAULT,
            signatures: vec![
                Signature::from(SIGNATURE_DEFAULT),
                Signature::from(EXTRA_SIGNATURE_DEFAULT),
            ],
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
                signatures: vec![
                    Signature::from("ipdisbeacon"),
                    Signature::from("pang-supremacy-maritime-revoke-afterglow")
                ]
            }
        );
    }
}
