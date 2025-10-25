use crate::feature::{clock, panels};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Clock(clock::Message),
    Panels(panels::Message),
}
