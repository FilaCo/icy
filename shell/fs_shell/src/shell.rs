use std::path::PathBuf;

use clap::Parser;
use directories::ProjectDirs;
use thiserror::Error;

use crate::{cli::Cli, config::Config};

#[derive(Debug)]
pub struct Shell {
    config_path: PathBuf,
}

impl Shell {
    pub fn new() -> Self {
        let args = Cli::parse();

        let config_path = args.config.unwrap_or(Self::default_config_path());

        Self { config_path }
    }

    pub fn run(self) -> Result<(), ShellError> {
        println!("{}", self.config_path.to_str().unwrap());
        Ok(())
    }

    fn default_config_path() -> PathBuf {
        Self::project_dirs().config_local_dir().join("config.toml")
    }

    fn project_dirs() -> ProjectDirs {
        ProjectDirs::from("com", "FilaCo", "FilaCo Shell").expect("unable to retrieve project dirs")
    }
}

impl Default for Shell {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Error, Debug)]
pub enum ShellError {}
