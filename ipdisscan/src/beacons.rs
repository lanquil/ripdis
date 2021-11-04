use color_eyre::Report;
use ipdisbeacon::bytes::Answer;
// use serde;
use std::net::IpAddr;
// use tracing::{debug, info, trace};

#[derive(Debug)]
pub struct BeaconAnswer {
    pub addr: IpAddr,
    pub payload: Answer,
}

pub fn run() -> Result<(), Report> {
    loop {
        todo!();
    }
}

pub fn put_in_queue(beacon_answer: BeaconAnswer) -> Result<(), Report> {
    // todo!();
    dbg!(beacon_answer);
    Ok(())
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::thread;
//     use std::time::Duration;
//     use tracing_test::traced_test;
//
//     #[test]
//     #[traced_test]
//     fn test_queue() {
//         todo!();
//     }
// }
