use iced::runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings;

use crate::config::Root;
use crate::shell::feature::{LayerSurfaceFeature, edges, wallpapers};
use crate::util::LayerSurfaceId;

#[derive(Debug, Clone)]
pub enum Message {
    ConfigLoaded(Root),
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
