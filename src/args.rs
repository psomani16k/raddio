use clap::{Parser, Subcommand};

/// A tiny fuzzy-input launcher: pick a station, type input, run its command.
#[derive(Debug, Parser)]
#[command(name = "raddio", version, about)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a default config file if one does not already exist.
    Init,
    /// Tune into a station: run its command with your input.
    Run {
        /// Name of the station (must match a `name` in the config).
        station: String,
    },
    /// List all stations form the config
    List,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }
}
