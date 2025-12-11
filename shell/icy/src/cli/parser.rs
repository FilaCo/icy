use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::style::CLI_STYLING;

#[derive(Parser, Debug)]
#[command(version, about, long_about, styles=CLI_STYLING)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,
    #[command(subcommand)]
    pub command: IcyCommand,
}

#[derive(Subcommand, Debug)]
pub enum IcyCommand {
    Shell {
        /// Detach icy shell from the current terminal
        #[arg(short, long)]
        detach: bool,
    },
}
