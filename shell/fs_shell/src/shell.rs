use clap::Parser;
use thiserror::Error;

use crate::cli::{Cli, Commands::*};

#[derive(Debug)]
pub struct Shell {}

impl Shell {
    pub fn new() -> Self {
        Self {}
    }

    // pub fn run(&mut self) -> Result<(), ShellError> {
    //     match self.args.command.unwrap_or_default() {
    //         Open { detach } => self.handle_open(detach),
    //         Close => self.handle_close(),
    //     }
    // }
    //
    // fn handle_open(&mut self, detach: bool) -> Result<(), ShellError> {}
    //
    // fn handle_close(&mut self) -> Result<(), ShellError> {}
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShellError {}
