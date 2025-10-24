mod action;
mod message;
mod window_descriptor;

use std::collections::HashMap;

pub use action::*;
use iced::window;
pub use message::*;
use window_descriptor::*;

#[derive(Debug)]
pub struct Windows {
    inner: HashMap<window::Id, WindowDescriptor>,
}

impl Windows {
    pub fn new() -> Self {
        let inner = HashMap::new();
        Self { inner }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Open => self.open(),
        }
    }

    fn open(&mut self) -> Action {
        Action::None
    }
}

impl Default for Windows {
    fn default() -> Self {
        Self::new()
    }
}
