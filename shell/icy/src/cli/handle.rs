use crate::cli::{Cli, Commands};

impl Cli {
    pub async fn handle(&mut self) {
        match self.command.take().unwrap_or_default() {
            Commands::Open { detach } => self.handle_open(detach),
            Commands::Close => self.handle_close(),
        }
    }

    fn handle_open(&mut self, _: bool) {}

    fn handle_close(&mut self) {}
}
