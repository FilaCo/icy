use crate::feature::{Feature, clock, panels};

#[derive(Debug, Clone)]
pub enum Message {
    OpenSurface(Feature),
    Clock(clock::Message),
    Panels(panels::Message),
}
