use clap::App;
use color_eyre::Report;
use std::thread;
use tracing::trace;

use ipdisscan::beacons;
use ipdisscan::broadcast;
use ipdisscan::listen;
use ipdisscan::setup::setup;

fn main() -> Result<(), Report> {
    setup()?;
    let matches = App::new("ipdisscan")
        .about("Search for active instances of ipdisbeacon and get system informations.")
        .arg_from_usage("-i, --tui 'Launch interactive TUI'")
        .get_matches();
    trace!(?matches);
    if matches.is_present("tui") {
        panic!("Not implemented yet");
    }
    thread::spawn(beacons::run);
    thread::spawn(listen::run);
    thread::spawn(broadcast::run);
    // order is important!
    Ok(())
}
