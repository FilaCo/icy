use iced::Task;
use icy_types::prelude::Message;

use crate::shell::Shell;

impl Shell {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        Task::none()
    }
}
