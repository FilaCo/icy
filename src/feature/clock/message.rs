use chrono::{DateTime, Local};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Set(DateTime<Local>),
}
