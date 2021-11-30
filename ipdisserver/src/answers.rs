use crate::bytes::{Answer, BeaconInfos};
use color_eyre::Report;
use gethostname::gethostname;
use serde_json;
use std::path::Path;
use tracing::trace;

pub fn get_answer(inventory_files: &[&Path]) -> Result<Answer, Report> {
    let basic_answer = get_basic_answer();
    trace!(?basic_answer);
    let inventory_answer = get_inventory_answer(inventory_files);
    trace!(?inventory_answer);
    let answer = Answer::from(serde_json::to_string(&join_answers(
        basic_answer,
        inventory_answer,
    ))?);
    Ok(answer)
}

fn join_answers(basic_answer: BeaconInfos, inventory_answer: Option<BeaconInfos>) -> BeaconInfos {
    todo!()
}

fn get_hostname() -> String {
    gethostname().to_string_lossy().into()
}

fn get_basic_answer() -> BeaconInfos {
    let hostname = BeaconInfos::String(get_hostname());
    let hostname_key = "hostname".to_string();
    let mut hostname_answer = serde_json::map::Map::new();
    hostname_answer.insert(hostname_key, hostname);
    BeaconInfos::Object(hostname_answer)
}

fn get_inventory_answer(inventory_files: &[&Path]) -> Option<BeaconInfos> {
    todo!();
    // execute safely
}
