use crate::{
    cli::{Cli, Commands},
    shell::State,
};

impl Cli {
    pub async fn handle(&mut self) {
        match self.command.take().unwrap_or_default() {
            Commands::Open { detach } => self.handle_open(detach),
            Commands::Close => self.handle_close(),
        }
    }

    fn handle_open(&mut self, _: bool) {
        iced::daemon(State::title, State::update, State::view)
            .run_with(State::new)
            .expect("something went wrong")
    }

    fn handle_close(&mut self) {}
}
