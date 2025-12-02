use iced::{Element, Task, widget::horizontal_rule, window};
use icy_types::prelude::ShellMessage;

use Shell::*;

pub enum Shell {
    Loading,
}

impl Shell {
    pub fn new() -> (Self, Task<ShellMessage>) {
        (Loading, Task::none())
    }

    pub fn namespace(&self) -> String {
        String::from("icy")
    }

    pub fn update(&mut self, msg: ShellMessage) -> Task<ShellMessage> {
        Task::none()
    }

    pub fn view(&self, _: window::Id) -> Element<ShellMessage> {
        horizontal_rule(2).into()
    }

    pub fn remove_id(&mut self, _: window::Id) {}
}
