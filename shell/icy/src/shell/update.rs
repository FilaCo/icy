use iced::{
    Task, platform_specific::shell::commands::layer_surface::get_layer_surface,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings,
};

use crate::shell::{
    Message, Shell, edges_action_to_message,
    feature::{LayerSurfaceFeature, edges, wallpapers},
    wallpapers_action_to_message,
};
use crate::util::LayerSurfaceId;

impl Shell {
    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::OpenLayerSurface { feat, settings } => self.open_surface(feat, settings),
            Message::LayerSurfaceOpened { id, feat } => self.layer_surface_opened(id, feat),
            Message::Edges(edges_msg) => edges_action_to_message(self.edges.update(edges_msg)),
            Message::Wallpapers(wallpapers_msg) => {
                wallpapers_action_to_message(self.wallpapers.update(wallpapers_msg))
            }
        }
    }

    fn open_surface(
        &mut self,
        feat: LayerSurfaceFeature,
        settings: SctkLayerSurfaceSettings,
    ) -> Task<Message> {
        let id = settings.id;
        // We can't just map the result task of `get_layer_surface()`, because it leads the following
        // not executed
        Task::batch(vec![
            get_layer_surface(settings),
            Task::done(Message::LayerSurfaceOpened { id, feat }),
        ])
    }

    fn layer_surface_opened(
        &mut self,
        id: LayerSurfaceId,
        feat: LayerSurfaceFeature,
    ) -> Task<Message> {
        self.layer_surface_features.insert(id, feat);
        match feat {
            LayerSurfaceFeature::Edges => {
                Task::done(Message::Edges(edges::Message::LayerSurfaceOpened(id)))
            }
            LayerSurfaceFeature::Wallpapers => Task::done(Message::Wallpapers(
                wallpapers::Message::LayerSurfaceOpened(id),
            )),
        }
    }
}
