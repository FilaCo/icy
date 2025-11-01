use crate::{
    feature::{Feature, clock, panels},
    util::LayerSurfaceId,
};

#[derive(Debug, Clone)]
pub enum Message {
    OpenLayerSurface(Feature),
    LayerSurfaceOpened { id: LayerSurfaceId, feat: Feature },
    Clock(clock::Message),
    Panels(panels::Message),
}
