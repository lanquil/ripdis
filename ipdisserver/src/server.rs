use crate::answers::get_answer;
use crate::answers::Answer;
use crate::conf::ServerConfig;
use crate::signature::Signature;
use color_eyre::eyre::Report;
use std::collections::HashSet;
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::path::Path;
use std::time::{Duration, SystemTime};
use tracing::{info, instrument, trace};

const RECV_BUFFER_LENGHT: usize = 128; // update ipdisserver and ipdisscan CLI documentation if changed
const RATE_LIMIT_TIMEOUT: Duration = Duration::from_secs(10); // do not accept more than a request every 10 s from each IP

#[instrument]
pub fn run(conf: &ServerConfig) -> Result<(), Report> {
    let socket = UdpSocket::bind(format!("{}:{}", conf.listening_addr, conf.port))?;
    info!(?socket, "Listening for scanner requests.");
    let clock = Clock::default();
    let mut rate_limiter = RateLimiter::new(&clock);
    loop {
        rate_limiter.conditional_reset();
        rate_limiter = serve_single(
            &socket,
            &conf.signatures,
            &conf.inventory_files,
            rate_limiter,
        )?;
    }
}

#[derive(Debug, Clone)]
struct RateLimiter<'a> {
    served_ips: HashSet<SocketAddr>,
    clock: &'a dyn WrappedSystemTime,
    next_reset: SystemTime,
}

impl<'a> RateLimiter<'a> {
    fn new(clock: &'a dyn WrappedSystemTime) -> Self {
        Self {
            served_ips: HashSet::default(),
            clock,
            next_reset: clock.now(),
        }
    }
}

impl RateLimiter<'_> {
    /// Return true if the address is not in served_ips, add it.
    fn check(&mut self, ip: &SocketAddr) -> bool {
        self.conditional_reset();
        let not_already_served = self.served_ips.insert(*ip);
        trace!(%ip, %not_already_served, "IP checked.");
        not_already_served
    }

    /// Reset served_ips if timeout has elapsed, set new timeout and return true. Return false
    /// otherwise.
    fn conditional_reset(&mut self) -> bool {
        let now = self.clock.now();
        if now >= self.next_reset {
            self.next_reset = now + RATE_LIMIT_TIMEOUT;
            self.served_ips = HashSet::default();
            trace!(?self.next_reset, "Cleared served IPs.");
            return true;
        }
        false
    }
}

trait WrappedSystemTime: std::fmt::Debug {
    // NB: supertrait
    fn now(&self) -> SystemTime;
}

#[derive(Debug, Default)]
struct Clock;

impl WrappedSystemTime for Clock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

#[derive(Debug)]
struct DummyClock {
    time: SystemTime,
}

impl WrappedSystemTime for DummyClock {
    fn now(&self) -> SystemTime {
        self.time
    }
}

#[instrument]
fn serve_single<'a>(
    socket: &UdpSocket,
    expected_signatures: &[Signature],
    inventory_files: &[&Path],
    mut rate_limiter: RateLimiter<'a>,
) -> Result<RateLimiter<'a>, Report> {
    let (addr, received) = receive(socket)?;
    if !is_signature_vaid(&received, expected_signatures) {
        trace!(%received, %addr, "Bad signature received, not answering.");
        return Ok(rate_limiter);
    };
    if !rate_limiter.check(&addr) {
        return Ok(rate_limiter);
    }
    let answer = get_answer(inventory_files)?;
    respond(socket, &addr, &answer)?;
    info!(%answer, %addr, "Answered.");
    Ok(rate_limiter)
}

fn is_signature_vaid(received: &Signature, expected: &[Signature]) -> bool {
    trace!(%received, ?expected, "Validating received signature.");
    for signature in expected.iter() {
        if signature == received {
            trace!(%received, "Received signature matches.");
            return true;
        }
    }
    false
}

fn receive(socket: &UdpSocket) -> Result<(SocketAddr, Signature), Report> {
    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = [0; RECV_BUFFER_LENGHT];
    trace!(?socket, "Listening.");
    let (lenght, source) = socket.recv_from(&mut buf)?;
    let received: Signature = (&buf[..lenght]).into();
    trace!(%lenght, %source, "Datagram received.");
    Ok((source, received))
}

fn respond(socket: &UdpSocket, addr: &SocketAddr, msg: &Answer) -> Result<(), Report> {
    socket.send_to(&msg.0, addr)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;
    use std::thread;
    use std::time::Duration;

    #[test]
    #[tracing_test::traced_test]
    fn test_serve_localhost() {
        let conf = ServerConfig::default();
        let sending_socket = UdpSocket::bind(format!("{}:{}", Ipv4Addr::UNSPECIFIED, 0)).unwrap();
        let receiving_socket = sending_socket
            .try_clone()
            .expect("couldn't clone the socket");
        let beacon_socket = UdpSocket::bind(format!("{}:{}", conf.listening_addr, 0)).unwrap();
        let server_port = beacon_socket.local_addr().unwrap().port();
        let conf_clone = conf.clone();
        let server_handle = thread::spawn(move || {
            let clock = Clock::default();
            serve_single(
                &beacon_socket,
                &conf_clone.signatures,
                &conf_clone.inventory_files,
                RateLimiter::new(&clock),
            )
            .unwrap();
        });
        let scanner_handle = thread::spawn(move || {
            thread::sleep(Duration::from_secs_f64(0.1));
            let beacon_addr = SocketAddr::from(([127, 0, 0, 1], server_port));
            sending_socket
                .send_to(conf.signatures.first().unwrap().0.as_ref(), beacon_addr)
                .unwrap();
            println!("[{}] <- {:?}", beacon_addr, &conf.signatures);
        });
        let response = receive(&receiving_socket).unwrap();
        println!("[{}] -> {}", response.0, response.1);
        server_handle.join().unwrap();
        scanner_handle.join().unwrap();
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_rate_limiter() {
        let time = SystemTime::now();
        let clock = DummyClock { time };
        let mut rate_limiter = RateLimiter::new(&clock);
        let ip = SocketAddr::from(([10, 11, 12, 13], 1234));
        assert!(rate_limiter.check(&ip));
        assert!(!rate_limiter.check(&ip));
        let time = SystemTime::now() + RATE_LIMIT_TIMEOUT + Duration::from_millis(1);
        let clock = DummyClock { time };
        rate_limiter.clock = &clock;
        assert!(rate_limiter.check(&ip));
        assert!(!rate_limiter.check(&ip));
    }
}
