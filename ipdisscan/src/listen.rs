use crate::beacons::{put_in_queue, BeaconAnswer};
use crate::broadcast::SCANNER_PORT;
use color_eyre::Report;
use ipdisbeacon::bytes::Answer;

use std::net::UdpSocket;
use tracing::{debug, info, trace};

const RECV_BUFFER_LENGHT: usize = 2usize.pow(10); // 1KiB
const LISTENING_ADDR: &str = "0.0.0.0";

pub fn run() -> Result<(), Report> {
    {
        let socket = socket_setup()?;
        info!(?socket, "Listening for beacon answers.");
        loop {
            serve_single(&socket)?;
        }
    }
}

fn socket_setup() -> Result<UdpSocket, Report> {
    let socket = UdpSocket::bind(format!("{}:{}", LISTENING_ADDR, SCANNER_PORT))?;
    Ok(socket)
}

fn serve_single(socket: &UdpSocket) -> Result<(), Report> {
    let beacon_answer = receive(socket)?;
    trace!(?beacon_answer.addr, %beacon_answer.payload, "Putting in queue.");
    put_in_queue(beacon_answer)?;
    Ok(())
}

fn receive(socket: &UdpSocket) -> Result<BeaconAnswer, Report> {
    let mut buf = [0; RECV_BUFFER_LENGHT];
    trace!(?socket, "Listening.");
    let (lenght, source) = socket.recv_from(&mut buf)?;
    let payload: Answer = (&buf[..lenght]).into();
    debug!(%lenght, %source, "Datagram received.");
    Ok(BeaconAnswer {
        addr: source.ip(),
        payload,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::SocketAddr;
    use std::thread;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_serve_localhost() {
        let payload = Answer("{\"an\": [\"example\", \"payload\"]}".as_bytes().to_vec());
        let expected = payload.clone();
        let listener_socket = UdpSocket::bind(format!("{}:{}", LISTENING_ADDR, 0)).unwrap();
        let listener_port = listener_socket.local_addr().unwrap().port();

        let sender_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f64(0.1));
            let sending_socket = UdpSocket::bind(format!("{}:{}", "0.0.0.0", 0)).unwrap();
            let listener_addr = SocketAddr::from(([127, 0, 0, 1], listener_port));
            sending_socket.send_to(&payload.0, listener_addr).unwrap();
            println!("[{}] -> {}", listener_addr, payload);
        });

        let answer = receive(&listener_socket).unwrap();
        assert_eq!(answer.payload.0, expected.0);
        sender_handle.join().unwrap();
    }
}
