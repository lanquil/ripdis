use crate::bytes::Signature;
use color_eyre::Report;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::net::Ipv4Addr;
use std::path::Path;
use tracing::info;

pub const SERVER_PORT_DEFAULT: u16 = 1901;
pub const SIGNATURE_DEFAULT: &str = "ipdisbeacon"; // must be shorter than RECV_BUFFER_LENGHT
const LISTENING_ADDR_DEFAULT: Ipv4Addr = Ipv4Addr::UNSPECIFIED; // "0.0.0.0"

#[derive(Debug, Clone, PartialEq)]
/// Server configurations.
pub struct ServerConfig {
    pub port: u16,
    pub listening_addr: Ipv4Addr,
    pub signatures: Vec<Signature>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: SERVER_PORT_DEFAULT,
            listening_addr: LISTENING_ADDR_DEFAULT,
            signatures: vec![Signature::from(SIGNATURE_DEFAULT)],
        }
    }
}

impl ServerConfig {
    /// Read a sequence of Signature from a file, one per line.
    /// Empty lines are ignored.
    pub fn parse_signatures_file(path: &Path) -> Result<Vec<Signature>, Report> {
        info!(?path, "Reading signatures from file.");
        let mut signatures = Vec::new();
        for line in read_file_lines(path)? {
            match line?.as_str() {
                "" => continue,
                s => signatures.push(Signature::from(s)),
            };
        }
        Ok(signatures)
    }
}

/// Returns an Iterator to the Reader of the lines of the file.
/// The output is wrapped in a Result to allow matching on errors
fn read_file_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[tracing_test::traced_test]
    fn test_conf() {
        let conf = ServerConfig::default();
        assert_eq!(
            conf,
            ServerConfig {
                port: 1901,
                listening_addr: Ipv4Addr::new(0, 0, 0, 0),
                signatures: vec![Signature::from("ipdisbeacon")]
            }
        );
    }

    #[test]
    #[tracing_test::traced_test]
    fn test_parse_signature_file() {
        let datadir = std::env::temp_dir()
            .as_path()
            .join("rust-test-ripdis-datadir/");
        // TODO: windows
        if let Err(error) = std::fs::create_dir(&datadir) {
            match error.kind() {
                std::io::ErrorKind::AlreadyExists => (),
                _ => panic!(),
            }
        };
        let empty_file_path = datadir.join("empty-file");
        let sign_file_path = datadir.join("sign-file");
        std::fs::write(&empty_file_path, "").unwrap();
        std::fs::write(&sign_file_path, "TestSignature\n\nsign line 2\n\n\n").unwrap();

        assert_eq!(
            ServerConfig::parse_signatures_file(&empty_file_path).unwrap(),
            Vec::new()
        );
        assert_eq!(
            ServerConfig::parse_signatures_file(&sign_file_path).unwrap(),
            vec![
                Signature::from("TestSignature"),
                Signature::from("sign line 2")
            ]
        );
    }
}
