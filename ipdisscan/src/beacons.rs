use color_eyre::Report;
use crossbeam::channel::{unbounded, Receiver, Sender};
use ipdisserver::bytes::Answer;
use std::collections::HashMap;
use std::fmt;
use std::net::IpAddr;
use std::thread;
use std::time::Duration;
use terminal_spinners::DOTS8 as SPINNER;
use terminal_spinners::{SpinnerBuilder, SpinnerHandle};
use tracing::{debug, trace};

use crossterm::{cursor, terminal, ExecutableCommand};
use std::io::stdout;

const PRINT_PERIOD: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
pub struct BeaconAnswer {
    pub addr: IpAddr,
    pub payload: Answer,
}

impl fmt::Display for BeaconAnswer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.addr, self.payload)
    }
}

type BeaconAnswers = HashMap<IpAddr, BeaconAnswer>;

pub fn run(channel_receiving_end: Receiver<BeaconAnswer>) -> Result<(), Report> {
    let mut beacons = BeaconAnswers::new();
    debug!(%PRINT_PERIOD, "Printing beacons.");

    let mut stdout = stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;
    let mut spinner_handle = get_spinner();
    loop {
        beacons = beacons_update(beacons, channel_receiving_end.clone())?;
        thread::sleep(Duration::from_secs_f64(PRINT_PERIOD));
        spinner_handle.stop_and_clear();
        stdout.execute(cursor::MoveTo(0, 0))?;
        print_beacons(beacons.values().cloned());
        spinner_handle = get_spinner();
        println!();
    }
}

pub fn init_channel() -> (Sender<BeaconAnswer>, Receiver<BeaconAnswer>) {
    unbounded()
}

fn get_spinner() -> SpinnerHandle {
    SpinnerBuilder::new()
        .spinner(&SPINNER)
        .text(" Looking for devices")
        .start()
}

fn beacons_update(
    mut beacons: BeaconAnswers,
    channel_receiving_end: Receiver<BeaconAnswer>,
) -> Result<BeaconAnswers, Report> {
    loop {
        let beacon = match channel_receiving_end.try_recv() {
            Ok(b) => b,
            _ => return Ok(beacons),
        };
        trace!(?beacon, "Updating beacons.");
        beacons.insert(beacon.addr, beacon);
    }
}

fn print_beacons<I>(beacons: I)
where
    I: Iterator<Item = BeaconAnswer>,
{
    println!("---");
    for beacon in beacons {
        println!("{}:", beacon.addr);
        println!("  - {}", beacon.payload);
    }
    println!("...");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::net::Ipv4Addr;

    #[test]
    #[tracing_test::traced_test]
    fn test_beacons_update() {
        let (sender, receiver) = init_channel();
        let answer1 = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            payload: Answer::default(),
        };
        let answer1_new = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 1)),
            payload: Answer::default(),
        };
        let answer2 = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)),
            payload: Answer::default(),
        };
        let answer2_new = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(192, 168, 0, 2)),
            payload: Answer::default(),
        };
        sender.send(answer2.clone()).unwrap();
        sender.send(answer1.clone()).unwrap();
        sender.send(answer1_new.clone()).unwrap();
        sender.send(answer2_new.clone()).unwrap();
        let mut beacons = BeaconAnswers::new();
        beacons = beacons_update(beacons, receiver).unwrap();
        assert_eq!(
            beacons.get(&answer1.addr).unwrap().payload,
            answer1_new.payload
        );
        assert_eq!(
            beacons.get(&answer2.addr).unwrap().payload,
            answer2_new.payload
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_put_in_queue() {
        let (sender, receiver) = init_channel();
        let an_answer = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            payload: Answer::default(),
        };
        sender.send(an_answer.clone()).unwrap();
        assert_eq!(receiver.try_recv().unwrap(), an_answer);
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_init_in_queue() {
        let (_sender, receiver) = init_channel();
        assert!(receiver.is_empty());
    }
}
