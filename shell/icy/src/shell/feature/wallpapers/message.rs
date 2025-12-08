use crate::shell::util::LayerSurfaceId;

#[derive(Debug, Clone)]
pub enum Message {
    LayerSurfaceOpened(LayerSurfaceId),
}
