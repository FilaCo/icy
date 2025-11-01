use iced::{
    Element, platform_specific::shell::commands::subsurface::Layer,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
};

use crate::{
    feature::panels::{Action, Message},
    util::LayerSurfaceId,
};

#[derive(Debug)]
pub struct Panels {}

impl Panels {
    pub fn new() -> (Self, Action) {
        (
            Self {},
            Action::OpenLayerSurface(SctkLayerSurfaceSettings {
                id: LayerSurfaceId::unique(),
                // TODO: size from monitors
                size: Some((Some(1920), Some(1080))),
                layer: Layer::Bottom,
                ..Default::default()
            }),
        )
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::LayerSurfaceOpened(_) => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}
