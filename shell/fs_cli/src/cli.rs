use std::path::PathBuf;

use clap::{
    Parser,
    builder::{
        Styles,
        styling::{AnsiColor, Effects, Style},
    },
};

#[derive(Parser, Debug)]
#[command(version, about, styles=CLI_STYLING)]
pub struct Cli {
    pub config: Option<PathBuf>,
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
