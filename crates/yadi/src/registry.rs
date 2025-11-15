use std::{
    any::{Any, TypeId},
    collections::BTreeMap,
    sync::Arc,
};

use thiserror::Error;

use crate::{entry::Entry, prelude::Injectable};

#[derive(Debug)]
pub struct Registry {
    inner: BTreeMap<TypeId, Box<dyn Any>>,
}

#[derive(Debug)]
pub struct RegistryBuilder {
    inner: BTreeMap<TypeId, Box<dyn Any>>,
}

#[derive(Error, Debug)]
pub enum RegistryError {}

impl Registry {
    pub fn builder() -> RegistryBuilder {
        RegistryBuilder::default()
    }

    pub fn resolve<T: Injectable>(&self) -> Arc<T> {
        self.inner
            .get(&TypeId::of::<T>())
            .expect("unable to resolve tag")
            .downcast_ref::<Entry<T>>()
            .expect("oopsie, wrong type associated with tag")
            .get(self)
    }

    pub(crate) fn new(inner: BTreeMap<TypeId, Box<dyn Any>>) -> Self {
        Self { inner }
    }
}

impl RegistryBuilder {
    pub fn new() -> Self {
        let inner = BTreeMap::new();
        Self { inner }
    }

    pub fn build(self) -> Result<Registry, RegistryError> {
        // TODO: validation
        Ok(Registry::new(self.inner))
    }

    pub fn register_lazy<T: Injectable>(self) -> Self {
        self.register(Entry::lazy(T::from_registry))
    }

    pub fn register_transient<T: Injectable>(self) -> Self {
        self.register(Entry::transient(T::from_registry))
    }

    fn register<T: Injectable>(mut self, entry: Entry<T>) -> Self {
        self.inner.insert(TypeId::of::<T>(), Box::new(entry));

        self
    }
}

impl Default for RegistryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {}
