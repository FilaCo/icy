use std::{collections::BTreeMap, sync::Arc, time::Duration};

use clap::Parser;
use iced::{
    Element, Subscription, Task,
    platform_specific::shell::commands::layer_surface::get_layer_surface, widget,
};

use crate::{
    Cli,
    app::Message,
    feature::{Clock, Feature, Panels, clock, panels},
    util::SurfaceId,
};

#[derive(Debug)]
pub struct Shell {
    sid_to_feat: BTreeMap<SurfaceId, Arc<Feature>>,
    clock: Clock,
    panels: Panels,
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

    pub fn title(&self, id: SurfaceId) -> String {
        "filaco_shell".into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::OpenSurface => self.open_surface(),
            Message::Clock(clock_msg) => clock_action_to_message(self.clock.update(clock_msg)),
            Message::Panels(panels_msg) => panels_action_to_message(self.panels.update(panels_msg)),
        }
    }

    pub fn view(&self, id: SurfaceId) -> Element<Message> {
        if let Some(feat) = self.sid_to_feat.get(&id).map(|f| f.as_ref()) {
            match feat {
                Feature::Clock(clock) => return clock.view().map(Message::Clock),
                Feature::Panels(panels) => return panels.view().map(Message::Panels),
            }
        }

        widget::horizontal_space().into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![self.clock.subscription().map(Message::Clock)])
    }

    fn open_surface(&mut self) -> Task<Message> {
        let id = SurfaceId::unique();
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
        panels::Action::OpenSurface => Task::done(Message::OpenSurface),
    }
}
