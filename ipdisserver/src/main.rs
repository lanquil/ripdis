use clap::{App, Arg};
use color_eyre::{eyre::Report, eyre::WrapErr};
use ipdisserver::conf::ServerConfig;
use ipdisserver::server;
use ipdisserver::setup::setup;
use std::net::Ipv4Addr;
use std::path::Path;
use std::str::FromStr;
use tracing::{debug, info, trace};

fn main() -> Result<(), Report> {
    const PORT_OPT: &str = "port";
    const ADDR_OPT: &str = "addr";
    const SIGNATURES_OPT: &str = "signatures";
    const INVENTORY_OPT: &str = "inventory";
    const JOURNALD_OPT: &str = "journald";
    let matches = App::new("ipdisserver")
        .version("0.1.1")
        .about("Answer with system info to ipdisscan broadcasts.")
        .arg(
            Arg::with_name(PORT_OPT)
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Listening port. Default: 1901.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ADDR_OPT)
                .short("a")
                .long("listening-addr")
                .value_name("ADDR")
                .help("Default: 0.0.0.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(SIGNATURES_OPT)
                .short("s")
                .long("signatures-file")
                .value_name("SIGNATURES_FILE")
                .help("Path of a file with accepted signatures, one per line. UTF-8 characters are allowed. Each signature length must be 128 bytes at most. If not specified a single signature is accepted: `ipdisbeacon`.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(INVENTORY_OPT)
                .short("f")
                .long("answer-file")
                .value_name("ANSWER_FILE")
                .help(r#"Specify a list of files to execute, the output will be added to the answer. The output must be in the format `key0=value0\nkey1=value1\n...`. Repeat the option for each file."#)
                .multiple(true)
                .number_of_values(1)
                .takes_value(true),
        )
        .arg(
            Arg::with_name(JOURNALD_OPT)
                .short("j")
                .long("log-to-journald")
                .required(false)
                .takes_value(false)
                .help("Send logs to systemd-journald instead of stderr.")
        )
        .get_matches();

    let do_log_to_journald = matches.is_present(JOURNALD_OPT);
    setup(do_log_to_journald)?;
    debug!("Tracing setup complete, starting IP discovery server.");
    trace!(?matches);

    let mut conf = ServerConfig::default();
    if matches.is_present(PORT_OPT) {
        conf.port = matches
            .value_of(PORT_OPT)
            .unwrap()
            .parse()
            .wrap_err("Invalid port given")?;
    }
    if matches.is_present(ADDR_OPT) {
        let addr = matches.value_of(ADDR_OPT).unwrap().parse::<String>()?;
        conf.listening_addr = Ipv4Addr::from_str(&addr).wrap_err("Invalid IP v4 given")?;
    }
    if matches.is_present(SIGNATURES_OPT) {
        conf.signatures = ServerConfig::parse_signatures_file(Path::new(
            matches.value_of(SIGNATURES_OPT).unwrap(),
        ))?;
        info!("Accepted signatures: {:?}", conf.signatures);
    }
    if matches.is_present(INVENTORY_OPT) {
        conf.inventory_files = matches
            .values_of(INVENTORY_OPT)
            .unwrap()
            .into_iter()
            .map(Path::new)
            .collect();
    }

    server::run(&conf)?;
    Ok(())
}
