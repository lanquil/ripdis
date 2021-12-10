use clap::{App, Arg};
use color_eyre::Report;
use ipdisserver::conf::ServerConfig;
use ipdisserver::server;
use ipdisserver::setup::setup;
use std::net::Ipv4Addr;
use std::path::Path;
use std::str::FromStr;
use tracing::{debug, info, trace};

fn main() -> Result<(), Report> {
    let matches = App::new("ipdisserver")
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
            Arg::with_name("signatures")
                .short("s")
                .long("signatures-file")
                .value_name("SIGNATURES_FILE")
                .help("Path of a file with accepted signatures, one per line. UTF-8 characters are allowed. Each signature length must be 128 bytes at most. If not specified a single signature is accepted: `ipdisbeacon`")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("inventory")
                .short("f")
                .long("answer-file")
                .value_name("ANSWER_FILE")
                .help("Specify a list of files to execute, the output will be added to the answer. The output must be in the format `key0=value0\nkey1=value1\n...`. Repeat the option for each file.")
                .multiple(true)
                .number_of_values(1)
                .takes_value(true),
        )
        .get_matches();

    setup()?;
    debug!("Tracing setup complete, starting IP discovery beacon.");
    trace!(?matches);

    let mut conf = ServerConfig::default();
    if matches.is_present("port") {
        conf.port = matches.value_of("port").unwrap().parse()?;
    }
    if matches.is_present("addr") {
        let addr = matches.value_of("addr").unwrap().parse::<String>()?;
        conf.listening_addr = Ipv4Addr::from_str(&addr)?;
    }
    if matches.is_present("signatures") {
        conf.signatures = ServerConfig::parse_signatures_file(Path::new(
            matches.value_of("signatures").unwrap(),
        ))?;
        info!("Accepted signatures: {:?}", conf.signatures);
    }
    if matches.is_present("inventory") {
        conf.inventory_files = matches
            .values_of("inventory")
            .unwrap()
            .into_iter()
            .map(Path::new)
            .collect();
    }

    server::run(&conf)?;
    Ok(())
}
