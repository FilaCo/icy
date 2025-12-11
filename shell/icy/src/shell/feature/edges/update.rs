use crate::shell::feature::edges::{Action, Edges, Message};

impl Edges {
    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::LayerSurfaceOpened(_) => Action::None,
        }
    }
}
