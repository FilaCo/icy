use iced::{Element, Task, widget::horizontal_space, window};

use crate::{
    app::Message,
    feature::{Feature, FeatureGallery},
};
use Feature::*;

#[derive(Debug)]
pub struct Shell {
    gallery: FeatureGallery,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        tracing_subscriber::fmt::init();
        let registry = FeatureGallery::new();
        (Self { gallery: registry }, Task::none())
    }

    pub fn title(&self, wid: window::Id) -> String {
        if let Some(f) = self.gallery.get(&wid) {
            match f {
                Clock(c) => c.title(),
            }
        } else {
            "FilaCo Shell".into()
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Clock(message) => todo!(),
        }
        Task::none()
    }

    pub fn view(&self, wid: window::Id) -> Element<Message> {
        if let Some(f) = self.gallery.get(&wid) {
            match f {
                Clock(c) => c.view().map(Message::Clock),
            }
        } else {
            horizontal_space().into()
        }
    }
}
