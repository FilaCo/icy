use iced::{Element, widget::horizontal_rule};

use crate::shell::feature::wallpapers::{Message, Wallpapers};

impl Wallpapers {
    pub fn view(&self) -> Element<Message> {
        horizontal_rule(2).into()
    }
}
