use crate::prelude::Registry;

pub type Factory<T> = fn(&Registry) -> T;
