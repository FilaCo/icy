use iced::runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::shell::{
    feature::{LayerSurfaceFeature, edges, wallpapers},
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
    Edges(edges::Message),
    Wallpapers(wallpapers::Message),
}
