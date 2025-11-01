use iced::{Task, runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings};

use crate::feature::edges::Message;

pub enum Action {
    None,
    Run(Task<Message>),
    OpenLayerSurface(SctkLayerSurfaceSettings),
}
