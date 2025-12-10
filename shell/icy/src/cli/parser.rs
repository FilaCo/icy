use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::cli::style::CLI_STYLING;

#[derive(Parser, Default, Debug)]
#[command(version, about, styles=CLI_STYLING)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub(crate) config: Option<PathBuf>,
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Open {
        /// Attached mode: Run icy in the current terminal
        #[arg(short, long)]
        attach: bool,
    },
    Close,
    Lock,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Open { attach: false }
    }
}
