use color_eyre::eyre::Report;
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[instrument]
pub fn setup() -> Result<(), Report> {
    install_stderr_tracing();
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

/// Logging levels configuration as per
/// https://docs.rs/tracing-subscriber/latest/tracing_subscriber/filter/struct.EnvFilter.html#directives
fn get_envfilter(default: &str) -> EnvFilter {
    EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new(default))
        .unwrap()
}
