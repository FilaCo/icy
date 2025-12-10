use std::path::PathBuf;

use tracing::info;

use crate::{
    cli::{Cli, Commands},
    directories::config,
    shell::Shell,
};

impl Cli {
    pub async fn handle(&mut self) {
        let config_path = self.config.take().unwrap_or(config().to_path_buf());
        match self.command.take().unwrap_or_default() {
            Commands::Open { detach } => self.handle_open(config_path, detach),
            Commands::Close => self.handle_close(),
            Commands::Lock => self.handle_lock(),
        }
    }

    fn handle_open(&mut self, config_path: PathBuf, detach: bool) {
        if detach {}

        iced::daemon(Shell::title, Shell::update, Shell::view)
            .run_with(|| Shell::new(config_path))
            .expect("unable to run iced::daemon")
    }

    fn handle_close(&mut self) {}

    fn handle_lock(&mut self) {}
}
