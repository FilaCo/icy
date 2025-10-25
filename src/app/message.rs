use crate::feature::clock;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Clock(clock::Message),
}
