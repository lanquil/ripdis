use clap::{App, Arg};
use color_eyre::Report;
use ipdisbeacon::bytes::Signature;
use ipdisbeacon::conf::BeaconConfig;
use ipdisbeacon::server;
use ipdisbeacon::setup::setup;
use std::net::Ipv4Addr;
use std::str::FromStr;
use tracing::debug;
use tracing::trace;

fn main() -> Result<(), Report> {
    setup()?;
    debug!("Tracing setup complete, starting IP discovery beacon.");
    let matches = App::new("ipdisbeacon")
        .version("0.1.0")
        .about("Answer with system info to ipdisscan broadcasts.")
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Listening port. Default: 1901")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("addr")
                .short("a")
                .long("listening-addr")
                .value_name("ADDR")
                .help("Default: 0.0.0.0")
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

    let mut conf = BeaconConfig::default();
    if matches.is_present("port") {
        conf.port = matches.value_of("port").unwrap().parse()?;
    }
    if matches.is_present("addr") {
        let addr = matches.value_of("addr").unwrap().parse::<String>()?;
        conf.listening_addr = Ipv4Addr::from_str(&addr)?;
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

    server::run(&conf)?;
    Ok(())
}
