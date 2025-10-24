pub mod clock;
mod gallery;

pub use gallery::*;

#[derive(Debug)]
pub enum Feature {
    Clock(clock::Clock),
}
