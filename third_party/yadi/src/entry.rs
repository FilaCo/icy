use std::sync::{Arc, OnceLock};

use crate::{
    factory::Factory,
    prelude::{Injectable, Registry},
};

#[derive(Debug)]
pub struct Entry<T: Injectable> {
    factory: Factory<T>,
    kind: EntryKind<T>,
}

#[derive(Debug)]
enum EntryKind<T: Injectable> {
    Transient,
    Lazy(OnceLock<Arc<T>>),
}

impl<T: Injectable> Entry<T> {
    pub fn lazy(factory: Factory<T>) -> Self {
        Self::new(factory, EntryKind::Lazy(OnceLock::new()))
    }

    pub fn transient(factory: Factory<T>) -> Self {
        Self::new(factory, EntryKind::Transient)
    }

    pub fn get(&self, registry: &Registry) -> Arc<T> {
        match &self.kind {
            EntryKind::Transient => Arc::new((self.factory)(registry)),
            EntryKind::Lazy(instance) => {
                Arc::clone(instance.get_or_init(|| Arc::new((self.factory)(registry))))
            }
        }
    }

    fn new(factory: Factory<T>, kind: EntryKind<T>) -> Self {
        Self { factory, kind }
    }
}
