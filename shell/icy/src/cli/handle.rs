use iced_sessionlock::{MultiApplication, settings::Settings};

use crate::{
    cli::{Cli, Commands},
    lock::Lock,
    shell::Shell,
};

impl Cli {
    pub async fn handle(&mut self) {
        match self.command.take().unwrap_or_default() {
            Commands::Open { detach } => self.handle_open(detach),
            Commands::Close => self.handle_close(),
            Commands::Lock => self.handle_lock(),
        }
    }

    fn handle_open(&mut self, _: bool) {
        // TODO: handle error
        iced_layershell::build_pattern::daemon(
            Shell::namespace,
            Shell::update,
            Shell::view,
            Shell::remove_id,
        )
        .run_with(Shell::new)
        .expect("something went wrong")
    }

    fn handle_close(&mut self) {}

    fn handle_lock(&mut self) {
        // TODO: handle error
        Lock::run(Settings::default()).expect("something went wrong")
    }
}
