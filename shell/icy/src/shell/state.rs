use std::collections::HashMap;

use iced::Task;

use crate::shell::{
    Message,
    feature::{
        LayerSurfaceFeature,
        edges::{self, Edges},
        wallpapers::{self, Wallpapers},
    },
    util::LayerSurfaceId,
};

#[derive(Debug)]
pub struct Shell {
    pub layer_surface_features: HashMap<LayerSurfaceId, LayerSurfaceFeature>,
    pub edges: Edges,
    pub wallpapers: Wallpapers,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        let mut tasks = vec![];

        // Init edges feature
        let (edges, edges_action) = Edges::new();
        tasks.push(edges_action_to_message(edges_action));

        // Init wallpapers feature
        let (wallpapers, wallpapers_action) = Wallpapers::new();
        tasks.push(wallpapers_action_to_message(wallpapers_action));

        (
            Self {
                layer_surface_features: HashMap::new(),
                edges,
                wallpapers,
            },
            Task::batch(tasks),
        )
    }

    pub fn title(&self, id: LayerSurfaceId) -> String {
        if let Some(feature) = self.layer_surface_features.get(&id) {
            match feature {
                LayerSurfaceFeature::Edges => self.edges.title(),
                LayerSurfaceFeature::Wallpapers => self.wallpapers.title(),
            }
        } else {
            String::from("icy")
        }
    }
}

pub fn edges_action_to_message(edges_action: edges::Action) -> Task<Message> {
    match edges_action {
        edges::Action::None => Task::none(),
        edges::Action::Run(task) => task.map(Message::Edges),
        edges::Action::OpenLayerSurface(settings) => Task::done(Message::OpenLayerSurface {
            feat: LayerSurfaceFeature::Edges,
            settings,
        }),
    }
}

pub fn wallpapers_action_to_message(wallpapers_action: wallpapers::Action) -> Task<Message> {
    match wallpapers_action {
        wallpapers::Action::None => Task::none(),
        wallpapers::Action::Run(task) => task.map(Message::Wallpapers),
        wallpapers::Action::OpenLayerSurface(settings) => Task::done(Message::OpenLayerSurface {
            feat: LayerSurfaceFeature::Wallpapers,
            settings,
        }),
    }
}
