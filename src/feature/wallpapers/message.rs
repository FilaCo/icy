use crate::util::LayerSurfaceId;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    LayerSurfaceOpened(LayerSurfaceId),
}
