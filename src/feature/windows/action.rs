use iced::Task;

use crate::feature::windows::Message;

pub enum Action {
    None,
    Run(Task<Message>),
}
