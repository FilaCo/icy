use iced::Task;

use crate::shell::{Message, Shell};

impl Shell {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        Task::none()
    }
}
