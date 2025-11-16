use std::{any::TypeId, fmt::Display};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Tag {}

impl From<TypeId> for Tag {
    fn from(value: TypeId) -> Self {
        todo!()
    }
}

impl Display for Tag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
