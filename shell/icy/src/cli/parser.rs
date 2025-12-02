use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::{cli::style::CLI_STYLING, directories};

#[derive(Parser, Default, Debug)]
#[command(version, about, styles=CLI_STYLING)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", default_value=directories::config().join("config.toml").into_os_string())]
    pub(crate) config: PathBuf,
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Open {
        /// Detached mode: Run icy in the background
        #[arg(short, long)]
        detach: bool,
    },
    Close,
    Lock,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Open { detach: true }
    }
}
