use clap::{App, Arg};
use color_eyre::Report;
use ipdisscan::beacons;
use ipdisscan::broadcast;
use ipdisscan::broadcast::socket_setup;
use ipdisscan::conf::ScannerConfig;
use ipdisscan::listen;
use ipdisscan::setup::setup;
use ipdisserver::signature::Signature;
use std::net::Ipv4Addr;
use std::str::FromStr;
use std::thread;
use tracing::trace;

fn main() -> Result<(), Report> {
    let matches = App::new("ipdisscan")
        .version("0.1.0")
        .about("Search for active instances of ipdisserver and get system informations.")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("scanner-source-port")
                .value_name("PORT")
                .help("UDP port used to receive ipdisserver answers. Default: 1902")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("target_port")
                .short("b")
                .long("broadcast-target-port")
                .value_name("TARGET-PORT")
                .help("ipdisserver listening UDP port. Default: 1901")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("broadcast-addr")
                .value_name("ADDR")
                .help("Broadcasting address. Default: 255.255.255.255")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("signature")
                .short("s")
                .long("signature")
                .value_name("SIGN")
                .multiple(true)
                .number_of_values(1)
                .help("Strings used to recognize ipdisserver instances. UTF-8 characters are allowed. Each signature length must be 128 bytes at most. This option can be used more than once. Default: `ipdisbeacon` and `pang-supremacy-maritime-revoke-afterglow` (the second one is for backward compatibility).")
                .takes_value(true),
        )
        .get_matches();

    setup()?;
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
        conf.signatures = matches
            .values_of("signature")
            .unwrap()
            .into_iter()
            .map(Signature::from)
            .collect();
        // replace default signatures
    }

    let socket = socket_setup(conf.port)?;
    let socket_c = socket.try_clone()?;
    let (channel_send_end, channel_receive_end) = beacons::init_channel();
    thread::spawn(move || listen::run(&socket_c, channel_send_end));
    thread::spawn(move || broadcast::run(&socket, &conf));
    beacons::run(channel_receive_end)?;
    Ok(())
}
