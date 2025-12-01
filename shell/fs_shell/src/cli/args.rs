use std::{default, path::PathBuf};

use clap::{
    Parser, Subcommand,
    builder::{
        Styles,
        styling::{AnsiColor, Effects, Style},
    },
};

use crate::directories;

#[derive(Parser, Default, Debug)]
#[command(version, about, styles=CLI_STYLING)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE", default_value=directories::config().join("config.toml").into_os_string())]
    pub config: PathBuf,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Open {
        /// Detached mode: Run icy in the background
        #[arg(short, long)]
        detach: bool,
    },
    Close,
}

impl Default for Commands {
    fn default() -> Self {
        Self::Open { detach: true }
    }
}

// const NOP: Style = Style::new();
const HEADER: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const USAGE: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const LITERAL: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const PLACEHOLDER: Style = AnsiColor::Cyan.on_default();
const ERROR: Style = AnsiColor::Red.on_default().effects(Effects::BOLD);
// const WARN: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);
// const NOTE: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
// const GOOD: Style = AnsiColor::Green.on_default().effects(Effects::BOLD);
const VALID: Style = AnsiColor::Cyan.on_default().effects(Effects::BOLD);
const INVALID: Style = AnsiColor::Yellow.on_default().effects(Effects::BOLD);

const CLI_STYLING: Styles = Styles::styled()
    .header(HEADER)
    .usage(USAGE)
    .literal(LITERAL)
    .placeholder(PLACEHOLDER)
    .error(ERROR)
    .valid(VALID)
    .invalid(INVALID);
