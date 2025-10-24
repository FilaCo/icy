use crate::feature::{clock, windows};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Clock(clock::Message),
    Windows(windows::Message),
}
