use iced::{Element, widget::horizontal_rule};

use crate::shell::feature::edges::{Edges, Message};

impl Edges {
    pub fn view(&self) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
