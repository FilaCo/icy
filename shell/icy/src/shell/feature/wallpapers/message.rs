use crate::util::LayerSurfaceId;

#[derive(Debug, Clone)]
pub enum Message {
    LayerSurfaceOpened(LayerSurfaceId),
}
