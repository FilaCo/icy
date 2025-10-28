use iced::{
    Element, Task, platform_specific::shell::commands::layer_surface::get_layer_surface,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
};

use crate::feature::panels::{Action, Message};

#[derive(Debug)]
pub struct Panels {}

impl Panels {
    pub fn new() -> (Self, Action) {
        todo!()
    }

    pub fn update(&mut self, msg: Message) -> Action {
        todo!()
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}
