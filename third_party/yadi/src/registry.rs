use std::{
    any::{Any, TypeId, type_name},
    collections::{BTreeMap, HashMap},
    sync::Arc,
};

use thiserror::Error;
use tracing::trace;

use crate::{entry::Entry, prelude::Tag};

#[derive(Debug)]
pub struct Registry {
    entries: HashMap<Tag, Entry>,
}

#[derive(Debug)]
pub struct RegistryBuilder {
    entries: HashMap<Tag, Entry>,
}

#[derive(Error, Debug)]
pub enum RegistryError {}

impl Registry {
    pub fn builder() -> RegistryBuilder {
        RegistryBuilder::default()
    }

    pub fn resolve<T: 'static>(&self) -> Arc<T> {
        self.resolve_by_tag(&TypeId::of::<T>().into())
    }

    pub fn resolve_by_tag<T: 'static>(&self, tag: &Tag) -> Arc<T> {
        trace!("resolve by tag {} for type: {}", tag, type_name::<T>());
        self.entries
            .get(tag)
            .expect("unable to find an entry")
            .get(self)
    }
}

impl RegistryBuilder {
    pub fn new() -> Self {
        todo!()
    }

    pub fn build(self) -> Result<Registry, RegistryError> {
        todo!()
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
