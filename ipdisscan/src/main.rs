use clap::App;
use color_eyre::Report;
use tracing::trace;

use ipdisscan::beacons;
use ipdisscan::broadcast;
use ipdisscan::broadcast::socket_setup;
use ipdisscan::listen;
use ipdisscan::setup::setup;
use std::thread;

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
    let socket = socket_setup()?;
    let socket_c = socket.try_clone()?;
    let queue = beacons::init_queue()?;
    let queue_c = queue.clone();
    thread::spawn(move || beacons::run(queue_c));
    thread::spawn(move || listen::run(&socket_c, queue));
    broadcast::run(&socket)?;
    Ok(())
}
