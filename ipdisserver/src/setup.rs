use color_eyre::eyre::Report;
use tracing::{error, instrument};
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[instrument]
pub fn setup(log_to_journald: bool) -> Result<(), Report> {
    match log_to_journald {
        false => install_stderr_tracing(),
        true => install_journald_tracing().unwrap_or_else(|_| {
            install_stderr_tracing();
            error!("Failed to connect to journald, logging to stderr.")
        }),
    };
    color_eyre::install()?;
    Ok(())
}

fn install_stderr_tracing() {
    let filter_layer = get_envfilter("warn");
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_writer(std::io::stderr);
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(ErrorLayer::default())
        .with(fmt_layer)
        .init();
}

fn install_journald_tracing() -> Result<(), Report> {
    let filter_layer = get_envfilter("info");
    let fmt_layer = tracing_journald::layer()?;
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(ErrorLayer::default())
        .with(fmt_layer)
        .init();
    Ok(())
}

/// Logging levels configuration as per
/// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
fn get_envfilter(default: &str) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default))
        .unwrap()
}
