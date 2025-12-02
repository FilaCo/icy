use crate::{
    cli::{Cli, Commands},
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
        tracing::error!("wtf!");
        // TODO: handle error
        iced::daemon(Shell::title, Shell::update, Shell::view)
            .run_with(Shell::new)
            .expect("something went wrong")
    }

    fn handle_close(&mut self) {}

    fn handle_lock(&mut self) {}
}
