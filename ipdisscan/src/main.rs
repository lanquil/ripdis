use clap::{App, Arg};
use color_eyre::Report;
use ipdisbeacon::bytes::Signature;
use ipdisscan::beacons;
use ipdisscan::broadcast;
use ipdisscan::broadcast::socket_setup;
use ipdisscan::conf::ScannerConfig;
use ipdisscan::listen;
use ipdisscan::setup::setup;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;
use tracing::trace;

fn main() -> Result<(), Report> {
    setup()?;
    let matches = App::new("ipdisscan")
        .version("0.1.0")
        .about("Search for active instances of ipdisbeacon and get system informations.")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("scanner-source-port")
                .value_name("PORT")
                .help("Default: 1902")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target_port")
                .short("b")
                .long("broadcast-target-port")
                .value_name("TARGET-PORT")
                .help("Default: 1901")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("broadcast-addr")
                .value_name("ADDR")
                .help("Default: 255.255.255.255")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("signature")
                .short("s")
                .long("signature")
                .value_name("SIGN")
                .help("Default: `ipdisbeacon`")
                .takes_value(true),
        )
        .get_matches();
    trace!(?matches);

    let mut conf = ScannerConfig::default();
    if matches.is_present("port") {
        conf.port = matches.value_of("port").unwrap().parse()?;
    }
    if matches.is_present("target_port") {
        conf.target_port = matches.value_of("target_port").unwrap().parse()?;
    }
    if matches.is_present("addr") {
        let str_broadcast_addr = matches.value_of("addr").unwrap().parse::<String>()?;
        conf.broadcast_addr = Ipv4Addr::from_str(&str_broadcast_addr)?;
    }
    if matches.is_present("signature") {
        conf.signature = Signature::from(
            matches
                .value_of("signature")
                .unwrap()
                .parse::<String>()?
                .as_str(),
        );
    }

    let socket = socket_setup(conf.port)?;
    let socket_c = socket.try_clone()?;
    let queue = beacons::init_queue()?;
    let queue_c = queue.clone();
    thread::spawn(move || listen::run(&socket_c, queue_c));
    thread::spawn(move || broadcast::run(&socket, &conf));
    beacons::run(queue)?;
    Ok(())
}
