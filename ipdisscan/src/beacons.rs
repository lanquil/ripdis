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
