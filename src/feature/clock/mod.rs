mod action;
mod message;

pub use action::*;
use iced::Element;
pub use message::*;

#[derive(Debug)]
pub struct Clock {}

impl Clock {
    pub fn title(&self) -> String {
        todo!()
    }

    pub fn update(&mut self, msg: Message) -> Action {
        todo!()
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}
