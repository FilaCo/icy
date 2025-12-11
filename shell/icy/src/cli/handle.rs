use std::path::PathBuf;

use daemonize::Daemonize;
use tracing::info;

use crate::{
    cli::{Cli, IcyCommand},
    directories::config,
    shell::Shell,
};

impl Cli {
    pub async fn handle(&mut self) {
        let config_path = self.config.take().unwrap_or(config().to_path_buf());
        match self.command {
            IcyCommand::Shell { detach } => self.handle_shell(config_path, detach),
        }
    }

    fn handle_shell(&mut self, config_path: PathBuf, attach: bool) {
        if !attach {
            info!("detach icy from the current terminal");
            Daemonize::new()
                .start()
                .expect("unable to detach icy from the current terminal");
        }

        iced::daemon(Shell::title, Shell::update, Shell::view)
            .run_with(|| Shell::new(config_path))
            .expect("unable to run iced::daemon")
    }
}
