use std::{
    any::Any,
    sync::{Arc, OnceLock},
};

use crate::prelude::Registry;

#[derive(Debug)]
pub struct Entry {
    factory: fn(&Registry) -> Box<dyn Any>,
    kind: EntryKind,
}

#[derive(Debug)]
enum EntryKind {
    Transient,
    Lazy(OnceLock<Arc<dyn Any>>),
}

impl Entry {
    pub fn lazy(factory: fn(&Registry) -> Box<dyn Any>) -> Self {
        Self::new(factory, EntryKind::Lazy(OnceLock::new()))
    }

    pub fn transient(factory: fn(&Registry) -> Box<dyn Any>) -> Self {
        Self::new(factory, EntryKind::Transient)
    }

    pub fn get(&self, registry: &Registry) -> Arc<dyn Any> {
        match &self.kind {
            EntryKind::Transient => Arc::new((self.factory)(registry)),
            EntryKind::Lazy(instance) => {
                Arc::clone(instance.get_or_init(|| Arc::new((self.factory)(registry))))
            }
        }
    }

    fn new(factory: fn(&Registry) -> Box<dyn Any>, kind: EntryKind) -> Self {
        Self { factory, kind }
    }
}
