use color_eyre::Report;
use ipdisbeacon::bytes::Answer;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::net::IpAddr;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use tracing::{debug, trace};

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

pub fn run(in_queue: Arc<Mutex<VecDeque<BeaconAnswer>>>) -> Result<(), Report> {
    let mut beacons = BeaconAnswers::new();
    debug!(%PRINT_PERIOD, "Printing beacons.");
    loop {
        beacons = beacons_update(beacons, in_queue.clone())?;
        thread::sleep(Duration::from_secs_f64(PRINT_PERIOD));
        print_beacons(beacons.values().cloned());
        trace!(?beacons, "Done printing beacons.");
    }
}

pub fn put_in_queue(
    beacon_answer: BeaconAnswer,
    in_queue: Arc<Mutex<VecDeque<BeaconAnswer>>>,
) -> Result<(), Report> {
    debug!(?beacon_answer, "Adding beacon answer to queue");
    in_queue
        .lock()
        .expect("Error accessing queue")
        .push_back(beacon_answer);
    Ok(())
}

pub fn init_queue() -> Result<Arc<Mutex<VecDeque<BeaconAnswer>>>, Report> {
    Ok(Arc::new(Mutex::new(VecDeque::new())))
}

fn beacons_update(
    mut beacons: BeaconAnswers,
    in_queue: Arc<Mutex<VecDeque<BeaconAnswer>>>,
) -> Result<BeaconAnswers, Report> {
    loop {
        let beacon = match in_queue.lock().expect("Error accessing queue").pop_front() {
            None => return Ok(beacons),
            Some(b) => b,
        };
        trace!(?beacon, "Updating beacons");
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
        let queue = init_queue().unwrap();
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
        put_in_queue(answer2.clone(), queue.clone()).unwrap();
        put_in_queue(answer1.clone(), queue.clone()).unwrap();
        put_in_queue(answer1_new.clone(), queue.clone()).unwrap();
        put_in_queue(answer2_new.clone(), queue.clone()).unwrap();
        let mut beacons = BeaconAnswers::new();
        beacons = beacons_update(beacons, queue).unwrap();
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
        let queue = init_queue().unwrap();
        let an_answer = BeaconAnswer {
            addr: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            payload: Answer::default(),
        };
        put_in_queue(an_answer.clone(), queue.clone()).unwrap();
        assert_eq!(queue.lock().unwrap().pop_front().unwrap(), an_answer);
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_init_in_queue() {
        let queue = init_queue().unwrap();
        assert!(queue.lock().unwrap().pop_front().is_none()); // expect empty queue
    }
}
