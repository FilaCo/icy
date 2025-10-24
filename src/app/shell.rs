use clap::Parser;
use iced::{Element, Task, widget::horizontal_space, window};

use crate::{
    Cli,
    app::Message,
    feature::{Feature, FeatureGallery, clock},
};
use Feature::*;

#[derive(Debug)]
pub struct Shell {
    gallery: FeatureGallery,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        tracing_subscriber::fmt::init();
        let _ = Cli::parse();
        let gallery = FeatureGallery::new();
        (Self { gallery }, Task::none())
    }

    pub fn title(&self, wid: window::Id) -> String {
        if let Some(f) = self.gallery.get(&wid) {
            match f {
                _ => {}
            }
        }

        "filaco_shell".into()
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Clock(clock_msg) => {
                if let Some(clock) = self.gallery.get_mut::<clock::Clock>() {
                    match clock.update(clock_msg) {
                        clock::Action::Run(task) => {
                            return task.map(Message::Clock);
                        }
                        _ => {}
                    }
                }
            }
            Message::Windows(win_msg) => todo!(),
        }
        Task::none()
    }

    pub fn view(&self, wid: window::Id) -> Element<Message> {
        if let Some(f) = self.gallery.get(&wid) {
            match f {
                Clock(c) => {
                    return c.view().map(Message::Clock);
                }
                _ => {}
            }
        }
        horizontal_space().into()
    }
}
