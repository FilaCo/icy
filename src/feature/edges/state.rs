use iced::{
    Color, Element, Theme, color, platform_specific::shell::commands::subsurface::Layer,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings, theme::Palette,
};

use crate::{
    feature::edges::{Action, Message},
    util::LayerSurfaceId,
};

#[derive(Debug)]
pub struct Edges {}

impl Edges {
    pub fn new() -> (Self, Action) {
        (
            Self {},
            Action::OpenLayerSurface(SctkLayerSurfaceSettings {
                id: LayerSurfaceId::unique(),
                // TODO: size from monitors
                size: Some((Some(1920), Some(1080))),
                layer: Layer::Bottom,
                namespace: "filaco_shell::edges".into(),
                ..Default::default()
            }),
        )
    }

    pub fn title(&self) -> String {
        "edges".into()
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::LayerSurfaceOpened(_) => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        "edges".into()
    }

    pub fn theme(&self) -> Theme {
        Theme::custom(
            "liquid".into(),
            Palette {
                background: Color::TRANSPARENT,
                text: color!(0x9aa5ce),    // Text
                primary: color!(0x2ac3de), // Blue
                success: color!(0x9ece6a), // Green
                danger: color!(0xf7768e),  // Red
            },
        )
    }
}
