use iced::{Task, runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings};

use crate::shell::feature::wallpapers::Message;

pub enum Action {
    None,
    Run(Task<Message>),
    OpenLayerSurface(SctkLayerSurfaceSettings),
}
