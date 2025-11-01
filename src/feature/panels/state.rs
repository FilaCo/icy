use iced::Element;

use crate::feature::{
    Clock,
    panels::{Action, Message},
};

#[derive(Debug)]
pub struct Panels<'shell> {
    clock: &'shell Clock,
}

impl<'shell> Panels<'shell> {
    pub fn new(clock: &'shell Clock) -> (Self, Action) {
        (Self { clock }, Action::OpenSurface)
    }

    pub fn update(&mut self, msg: Message) -> Action {
        todo!()
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}
