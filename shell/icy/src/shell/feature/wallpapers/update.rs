use crate::shell::feature::wallpapers::{Action, Message, Wallpapers};

impl Wallpapers {
    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::LayerSurfaceOpened(_) => Action::None,
        }
    }
}
