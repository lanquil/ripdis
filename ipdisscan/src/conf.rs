use ipdisbeacon::bytes::Signature;
use ipdisbeacon::server::SERVER_PORT as SERVER_PORT_DEFAULT;
use ipdisbeacon::server::SIGNATURE_DEFAULT;
use std::net::Ipv4Addr;

const SCANNER_PORT_DEFAULT: u16 = 1902;
const SCAN_PERIOD_DEFAULT: f64 = 1.0;
const BROADCAST_ADDR_DEFAULT: Ipv4Addr = Ipv4Addr::BROADCAST; // 255.255.255.255

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
