use clap::Parser;

/// A tiny fuzzy-input launcher: pick a station, type input, run its command.
#[derive(Debug, Parser)]
#[command(name = "raddio", version, about)]
pub struct Args {
    /// Name of the station to tune into (must match a `name` in the config).
    pub station: String,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
