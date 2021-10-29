use color_eyre::Report;
use ipdisbeacon::server;
use ipdisbeacon::setup::setup;
use tracing::debug;

fn main() -> Result<(), Report> {
    setup()?;
    debug!("Tracing setup complete, starting IP discovery beacon.");
    server::run()?;
    Ok(())
}
