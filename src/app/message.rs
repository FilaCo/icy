use iced::runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::{
    feature::{Feature, clock, panels},
    util::LayerSurfaceId,
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenLayerSurface {
        feat: Feature,
        settings: SctkLayerSurfaceSettings,
    },
    LayerSurfaceOpened {
        id: LayerSurfaceId,
        feat: Feature,
    },
    Clock(clock::Message),
    Panels(panels::Message),
}
