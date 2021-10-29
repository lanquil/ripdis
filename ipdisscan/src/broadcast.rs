use color_eyre::Report;
use ipdisbeacon::server::{SERVER_PORT, SIGNATURE_DEFAULT};
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use tracing::{info, trace};

pub const SCANNER_PORT: u16 = 1902;
const SCAN_PERIOD: f64 = 1.0;

pub fn run() -> Result<(), Report> {
    let frequency = 1.0 / SCAN_PERIOD;
    {
        let socket = socket_setup()?;
        info!(?socket, %frequency, "Scanning for beacons.");
        loop {
            send_single(&socket)?;
            wait_duty_cycle(SCAN_PERIOD);
        }
    }
}

fn socket_setup() -> Result<UdpSocket, Report> {
    let socket = UdpSocket::bind(format!("{}:{}", "0.0.0.0", SCANNER_PORT))
        .expect("Failed to setup broadcasting socket");
    // broadcast
    Ok(socket)
}

fn send_single(socket: &UdpSocket) -> Result<(), Report> {
    let beacon_broadcast_addr = SocketAddr::from(([255, 255, 255, 255], SERVER_PORT));
    socket
        .send_to(SIGNATURE_DEFAULT.0, beacon_broadcast_addr)
        .expect("Failed broadcasting signature");
    trace!(
        dest = %beacon_broadcast_addr,
        payload = ?SIGNATURE_DEFAULT.0,
        "Broadcasted."
    );
    Ok(())
}

fn wait_duty_cycle(scan_period: f64) {
    thread::sleep(Duration::from_secs_f64(scan_period));
}

#[cfg(test)]
mod test {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_send() {
        todo!();
    }
}