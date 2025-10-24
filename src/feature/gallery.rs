use std::collections::HashMap;

use iced::window;

use crate::feature::Feature;

#[derive(Debug)]
pub struct FeatureGallery {
    inner: HashMap<window::Id, Feature>,
}

impl FeatureGallery {
    pub fn new() -> Self {
        let inner = HashMap::new();
        Self { inner }
    }

    pub fn get(&self, wid: &window::Id) -> Option<&Feature> {
        self.inner.get(wid)
    }

    pub fn get_mut(&mut self, wid: &window::Id) -> Option<&mut Feature> {
        self.inner.get_mut(wid)
    }
}

impl Default for FeatureGallery {
    fn default() -> Self {
        Self::new()
    }
}
