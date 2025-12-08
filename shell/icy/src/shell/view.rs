use iced::{Element, widget::horizontal_rule, window};

use crate::shell::{Message, Shell};

impl Shell {
    pub fn view(&self, id: window::Id) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
