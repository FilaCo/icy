use Shell::*;
use iced::{Task, window};
use icy_types::prelude::Message;

#[derive(Debug)]
pub enum Shell {
    Loading,
    Loaded(State),
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        (Loading, Task::none())
    }

    pub fn title(&self, id: window::Id) -> String {
        String::from("icy")
    }
}

#[derive(Debug)]
pub struct State {}
