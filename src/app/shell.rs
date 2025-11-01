use std::{collections::BTreeMap, time::Duration};

use clap::Parser;
use iced::{
    Element, Subscription, Task, Theme,
    platform_specific::shell::commands::layer_surface::get_layer_surface,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings, widget,
};

use crate::{
    Cli,
    app::Message,
    feature::{Clock, Edges, LayerSurfaceFeature, Wallpapers, clock, edges, wallpapers},
    util::LayerSurfaceId,
};

#[derive(Debug)]
pub struct Shell {
    sid_to_feat: BTreeMap<LayerSurfaceId, LayerSurfaceFeature>,
    clock: Clock,
    edges: Edges,
    wallpapers: Wallpapers,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        tracing_subscriber::fmt::init();
        let _ = Cli::parse();

        let sid_to_feat = BTreeMap::new();
        let mut tasks = vec![];

        // Init clock feature
        // TODO: set duration from config
        let (clock, clock_action) = Clock::new(Duration::from_secs(1));
        tasks.push(clock_action_to_message(clock_action));

        // Init edges feature
        let (edges, edges_action) = Edges::new();
        tasks.push(edges_action_to_message(edges_action));

        let (wallpapers, wallpapers_action) = Wallpapers::new();
        tasks.push(wallpapers_action_to_message(wallpapers_action));

        (
            Self {
                sid_to_feat,
                clock,
                edges,
                wallpapers,
            },
            Task::batch(tasks),
        )
    }

    pub fn title(&self, id: LayerSurfaceId) -> String {
        match self.sid_to_feat.get(&id).expect("feature not found") {
            LayerSurfaceFeature::Edges => self.edges.title(),
            LayerSurfaceFeature::Wallpapers => self.wallpapers.title(),
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::OpenLayerSurface { feat, settings } => self.open_surface(feat, settings),
            Message::LayerSurfaceOpened { id, feat } => self.layer_surface_opened(id, feat),
            Message::Clock(clock_msg) => clock_action_to_message(self.clock.update(clock_msg)),
            Message::Edges(panels_msg) => edges_action_to_message(self.edges.update(panels_msg)),
            Message::Wallpapers(wallpapers_msg) => {
                wallpapers_action_to_message(self.wallpapers.update(wallpapers_msg))
            }
        }
    }

    pub fn view(&self, id: LayerSurfaceId) -> Element<Message> {
        match self.sid_to_feat.get(&id).expect("feature not found") {
            LayerSurfaceFeature::Edges => self.edges.view().map(Message::Edges),
            LayerSurfaceFeature::Wallpapers => self.wallpapers.view().map(Message::Wallpapers),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![self.clock.subscription().map(Message::Clock)])
    }

    pub fn theme(&self, id: LayerSurfaceId) -> Theme {
        match self.sid_to_feat.get(&id).expect("feature not found") {
            LayerSurfaceFeature::Edges => self.edges.theme(),
            LayerSurfaceFeature::Wallpapers => self.wallpapers.theme(),
        }
    }

    fn open_surface(
        &mut self,
        feat: LayerSurfaceFeature,
        settings: SctkLayerSurfaceSettings,
    ) -> Task<Message> {
        let id = settings.id;
        // We can't just map the result task of get_layer_surface(), because it leads the following
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
        self.sid_to_feat.insert(id, feat);
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

fn clock_action_to_message(clock_action: clock::Action) -> Task<Message> {
    match clock_action {
        clock::Action::None => Task::none(),
        clock::Action::Run(task) => task.map(Message::Clock),
    }
}

fn edges_action_to_message(edges_action: edges::Action) -> Task<Message> {
    match edges_action {
        edges::Action::None => Task::none(),
        edges::Action::Run(task) => task.map(Message::Edges),
        edges::Action::OpenLayerSurface(settings) => Task::done(Message::OpenLayerSurface {
            feat: LayerSurfaceFeature::Edges,
            settings,
        }),
    }
}

fn wallpapers_action_to_message(wallpapers_action: wallpapers::Action) -> Task<Message> {
    match wallpapers_action {
        wallpapers::Action::None => Task::none(),
        wallpapers::Action::Run(task) => task.map(Message::Wallpapers),
        wallpapers::Action::OpenLayerSurface(settings) => Task::done(Message::OpenLayerSurface {
            feat: LayerSurfaceFeature::Wallpapers,
            settings,
        }),
    }
}
