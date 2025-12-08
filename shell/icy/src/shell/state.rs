use std::collections::HashMap;

use iced::{Task, window};

use crate::shell::{Message, feature::WindowFeature};

#[derive(Debug)]
pub struct Shell {
    windows_features: HashMap<window::Id, WindowFeature>,
}

impl Shell {
    pub fn new() -> (Self, Task<Message>) {
        let mut tasks = vec![];
        (
            Self {
                windows_features: HashMap::new(),
            },
            Task::batch(tasks),
        )
    }

    pub fn title(&self, id: window::Id) -> String {
        if let Some(feature) = self.windows_features.get(&id) {
            match feature {
                WindowFeature::Edges => todo!(),
            }
        } else {
            String::from("icy")
        }
    }
}
