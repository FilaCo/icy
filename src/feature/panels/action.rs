use iced::Task;

use crate::feature::panels::Message;

pub enum Action {
    None,
    Run(Task<Message>),
    OpenSurface,
}
