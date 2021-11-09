use color_eyre::Report;
use ipdisbeacon::server::{SERVER_PORT, SIGNATURE_DEFAULT};
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;
use tracing::{info, trace};

const SCANNER_ADDR: &str = "0.0.0.0";
const SCANNER_PORT: u16 = 1902;
const SCAN_PERIOD: f64 = 1.0;
const BROADCASTING_ADDR: [u8; 4] = [255, 255, 255, 255];

pub fn run(socket: &UdpSocket) -> Result<(), Report> {
    let frequency = 1.0 / SCAN_PERIOD;
    {
        info!(?socket, %frequency, "Scanning for beacons.");
        loop {
            send_single(socket, SERVER_PORT)?;
            wait_duty_cycle(SCAN_PERIOD);
        }
    }
}

pub fn socket_setup() -> Result<UdpSocket, Report> {
    let socket = UdpSocket::bind(format!("{}:{}", SCANNER_ADDR, SCANNER_PORT))
        .expect("Failed to setup broadcasting socket");
    socket.set_broadcast(true)?;
    Ok(socket)
}

fn send_single(socket: &UdpSocket, port: u16) -> Result<(), Report> {
    let beacon_broadcast_addr = SocketAddr::from((BROADCASTING_ADDR, port));
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

    #[test]
    #[tracing_test::traced_test]
    fn test_send() {
        let listener_socket = UdpSocket::bind(format!("{}:{}", "0.0.0.0", 0)).unwrap();
        let mut buf = [0; SIGNATURE_DEFAULT.0.len()];
        let listener_port = listener_socket.local_addr().unwrap().port();

        let sender_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f64(0.1));
            let socket = socket_setup().unwrap();
            send_single(&socket, listener_port).unwrap();
        });

        let (lenght, _source) = listener_socket.recv_from(&mut buf).unwrap();
        assert_eq!(lenght, SIGNATURE_DEFAULT.0.len());
        assert_eq!(buf, SIGNATURE_DEFAULT.0);
        sender_handle.join().unwrap();
    }
}
