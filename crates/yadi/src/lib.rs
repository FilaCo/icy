mod entry;
mod factory;
pub mod injectable;
pub mod registry;

pub mod prelude {
    pub use crate::injectable::*;
    pub use crate::registry::*;

    #[cfg(feature = "derive")]
    pub use yadi_derive::*;
}

#[cfg(feature = "derive")]
pub use yadi_derive::*;
