use iced::{Element, widget::horizontal_rule};

use crate::shell::{Message, Shell, feature::LayerSurfaceFeature, util::LayerSurfaceId};

impl Shell {
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
