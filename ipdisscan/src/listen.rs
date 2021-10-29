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
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_serve_localhost() {
        let listener_socket =
            UdpSocket::bind(format!("{}:{}", LISTENING_ADDR, SCANNER_PORT)).unwrap();
        let listener_handle = thread::spawn(move || {
            serve_single(&listener_socket).unwrap();
        });
        let sending_port = 1901;
        let sending_socket = UdpSocket::bind(format!("{}:{}", "0.0.0.0", sending_port)).unwrap();
        let listener_addr = SocketAddr::from(([127, 0, 0, 1], SCANNER_PORT));
        let payload = Answer("{\"an\": [\"example\", \"payload\"]}".as_bytes().to_vec());
        sending_socket.send_to(&payload.0, listener_addr).unwrap();
        println!("[{}] -> {}", listener_addr, payload);
        listener_handle.join().unwrap();
        // assert_eq!(queue.get().unwrap().payload, payload);
        assert!(false);
    }
}
