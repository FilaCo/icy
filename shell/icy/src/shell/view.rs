use iced::Element;

use crate::shell::State;
use crate::shell::{Message, Shell, feature::LayerSurfaceFeature};
use crate::util::LayerSurfaceId;

impl Shell {
    pub fn view(&self, id: LayerSurfaceId) -> Element<Message> {
        match self {
            Shell::LoadingConfig => todo!(),
            Shell::Loaded(state) => state.view(id),
        }
    }
}

impl State {
    pub fn view(&self, id: LayerSurfaceId) -> Element<Message> {
        match self
            .layer_surface_features
            .get(&id)
            .expect("feature not found")
        {
            LayerSurfaceFeature::Edges => self.edges.view().map(Message::Edges),
            LayerSurfaceFeature::Wallpapers => self.wallpapers.view().map(Message::Wallpapers),
        }
    }
}
