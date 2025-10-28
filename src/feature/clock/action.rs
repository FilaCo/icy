use crate::feature::clock::Message;
use iced::Task;

pub enum Action {
    None,
    Run(Task<Message>),
}
