use iced::{Element, Task, core::window, widget::horizontal_rule};
use icy_msg::prelude::Message;

use State::*;

pub enum State {
    Loading,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        (Loading, Task::none())
    }

    pub fn title(&self, _: window::Id) -> String {
        "icy".to_string()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self, _: window::Id) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
