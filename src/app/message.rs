use iced::runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::{
    feature::{LayerSurfaceFeature, clock, edges, wallpapers},
    util::LayerSurfaceId,
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenLayerSurface {
        feat: LayerSurfaceFeature,
        settings: SctkLayerSurfaceSettings,
    },
    LayerSurfaceOpened {
        id: LayerSurfaceId,
        feat: LayerSurfaceFeature,
    },
    Clock(clock::Message),
    Edges(edges::Message),
    Wallpapers(wallpapers::Message),
}
