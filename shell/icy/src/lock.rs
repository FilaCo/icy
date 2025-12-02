use iced::{Element, Task, Theme, executor, widget::horizontal_rule};
use iced_sessionlock::MultiApplication;
use icy_types::prelude::LockMessage;

use Lock::*;

pub enum Lock {
    Loading,
}

impl MultiApplication for Lock {
    type Executor = executor::Default;

    type Message = LockMessage;

    type Flags = ();

    type Theme = Theme;

    fn new(_: Self::Flags) -> (Self, Task<Self::Message>) {
        (Self::Loading, Task::none())
    }

    fn namespace(&self) -> String {
        String::from("icy::lock")
    }

    fn update(&mut self, message: Self::Message) -> Task<Self::Message> {
        Task::none()
    }

    fn view(&self, window: iced::window::Id) -> Element<Self::Message> {
        horizontal_rule(2).into()
    }
}
