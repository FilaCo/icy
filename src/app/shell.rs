use std::{collections::BTreeMap, sync::Arc, time::Duration};

use clap::Parser;
use iced::{Element, Subscription, Task, widget};

use crate::{
    Cli,
    app::Message,
    feature::{Clock, Feature, Panels},
    util::SurfaceId,
};

#[derive(Debug)]
pub struct Shell {
    sid_to_feat: BTreeMap<SurfaceId, Arc<Feature>>,
    clock: Clock,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        tracing_subscriber::fmt::init();
        let _ = Cli::parse();

        let sid_to_feat = BTreeMap::new();
        let clock = Clock::new(Duration::from_secs(1));
        let panels = Panels::new();
        (Self { sid_to_feat, clock }, Task::none())
    }

    pub fn title(&self, id: SurfaceId) -> String {
        "filaco_shell".into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Clock(clock_msg) => self.clock.update(clock_msg).map(Message::Clock),
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
}
