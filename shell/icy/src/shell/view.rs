use iced::{Element, widget::horizontal_rule, window};
use icy_types::prelude::Message;

use crate::shell::Shell;

impl Shell {
    pub fn view(&self, id: window::Id) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
