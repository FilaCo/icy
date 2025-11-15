use crate::prelude::Registry;

pub trait Injectable: 'static {
    fn from_registry(registry: &Registry) -> Self;
}
