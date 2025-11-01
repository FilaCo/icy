use std::{collections::BTreeMap, time::Duration};

use clap::Parser;
use iced::{
    Element, Subscription, Task,
    platform_specific::shell::commands::layer_surface::get_layer_surface,
    runtime::platform_specific::wayland::layer_surface::SctkLayerSurfaceSettings, widget,
};

use crate::{
    Cli,
    app::Message,
    feature::{Clock, Feature, Panels, clock, panels},
    util::LayerSurfaceId,
};

#[derive(Debug)]
pub struct Shell<'shell> {
    sid_to_feat: BTreeMap<LayerSurfaceId, Feature>,
    clock: Clock,
    panels: Panels<'shell>,
}

impl<'shell> Shell<'shell> {
    pub fn new() -> (Self, Task<Message>) {
        tracing_subscriber::fmt::init();
        let _ = Cli::parse();

        let sid_to_feat = BTreeMap::new();
        let mut tasks = vec![];

        // Init clock feature
        // TODO: set duration from config
        let (clock, clock_action) = Clock::new(Duration::from_secs(1));
        tasks.push(clock_action_to_message(clock_action));

        // Init panels feature
        let (panels, panels_action) = Panels::new();
        tasks.push(panels_action_to_message(panels_action));

        (
            Self {
                sid_to_feat,
                clock,
                panels,
            },
            Task::batch(tasks),
        )
    }

    pub fn title(&self, id: LayerSurfaceId) -> String {
        "filaco_shell".into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::OpenLayerSurface(feat) => self.open_surface(feat),
            Message::LayerSurfaceOpened { id, feat } => self.layer_surface_opened(id, feat),
            Message::Clock(clock_msg) => clock_action_to_message(self.clock.update(clock_msg)),
            Message::Panels(panels_msg) => panels_action_to_message(self.panels.update(panels_msg)),
        }
    }

    pub fn view(&self, id: LayerSurfaceId) -> Element<Message> {
        if let Some(feat) = self.sid_to_feat.get(&id) {
            match feat {
                Feature::Clock => return self.clock.view().map(Message::Clock),
                Feature::Panels => return self.panels.view().map(Message::Panels),
            }
        }

        widget::horizontal_space().into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![self.clock.subscription().map(Message::Clock)])
    }

    fn open_surface(&mut self, feat: Feature) -> Task<Message> {
        let id = LayerSurfaceId::unique();

        get_layer_surface(SctkLayerSurfaceSettings {
            id,
            ..Default::default()
        })
        .map(move |id| Message::LayerSurfaceOpened { id, feat })
    }

    fn layer_surface_opened(&mut self, id: LayerSurfaceId, feat: Feature) -> Task<Message> {
        self.sid_to_feat.insert(id, feat);
        match feat {
            Feature::Clock => todo!(),
            Feature::Panels => Task::done(Message::Panels(panels::Message::LayerSurfaceOpened(id))),
        }
    }
}

fn clock_action_to_message(clock_action: clock::Action) -> Task<Message> {
    match clock_action {
        clock::Action::None => Task::none(),
        clock::Action::Run(task) => task.map(Message::Clock),
    }
}

fn panels_action_to_message(panels_action: panels::Action) -> Task<Message> {
    match panels_action {
        panels::Action::None => Task::none(),
        panels::Action::Run(task) => task.map(Message::Panels),
        panels::Action::OpenSurface => Task::done(Message::OpenLayerSurface(Feature::Panels)),
    }
}
