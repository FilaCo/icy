use iced::{Element, Task, widget::horizontal_rule, window};
use icy_types::prelude::Message;

use Shell::*;

pub enum Shell {
    Loading,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        tracing::info!("wtf!1");
        (Self::Loading, Task::none())
    }

    pub fn title(&self, _: window::Id) -> String {
        String::from("icy")
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        tracing::error!("lol");
        Task::none()
    }

    pub fn view(&self, id: window::Id) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
