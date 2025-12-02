pub mod cli;
mod config;
mod directories;
mod shell;

pub mod prelude {
    pub use crate::cli::*;
}
