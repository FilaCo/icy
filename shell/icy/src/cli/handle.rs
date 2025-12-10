use std::path::PathBuf;

use daemonize::Daemonize;
use tracing::{info, trace};

use crate::{
    cli::{Cli, Commands},
    directories::config,
    shell::Shell,
};

impl Cli {
    pub async fn handle(&mut self) {
        let config_path = self.config.take().unwrap_or(config().to_path_buf());
        match self.command.take().unwrap_or_default() {
            Commands::Open { attach } => self.handle_open(config_path, attach),
            Commands::Close => self.handle_close(),
            Commands::Lock => self.handle_lock(),
        }
    }

    fn handle_open(&mut self, config_path: PathBuf, attach: bool) {
        if !attach {
            trace!("detach icy from the current terminal");
            Daemonize::new()
                .start()
                .expect("unable to detach icy from the current terminal");
        }

        iced::daemon(Shell::title, Shell::update, Shell::view)
            .run_with(|| Shell::new(config_path))
            .expect("unable to run iced::daemon")
    }

    fn handle_close(&mut self) {}

    fn handle_lock(&mut self) {}
}
