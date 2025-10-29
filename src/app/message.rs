use crate::feature::{clock, panels};

#[derive(Debug, Copy, Clone)]
pub enum Message {
    OpenSurface,
    Clock(clock::Message),
    Panels(panels::Message),
}
