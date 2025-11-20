use clap::Parser;
use thiserror::Error;

use crate::cli::Cli;

#[derive(Debug)]
pub struct Shell {}

impl Shell {
    pub fn new() -> Self {
        let _ = Cli::parse();
        Self {}
    }

    pub fn run(self) -> Result<(), ShellError> {
        todo!()
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShellError {}
