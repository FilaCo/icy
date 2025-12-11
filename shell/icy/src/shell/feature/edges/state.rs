use iced::{
    platform_specific::shell::commands::subsurface::Layer,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
};

use crate::shell::feature::edges::Action;
use crate::util::LayerSurfaceId;

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
                namespace: "icy::edges".into(),
                ..Default::default()
            }),
        )
    }

    pub fn title(&self) -> String {
        String::from("edges")
    }
}
