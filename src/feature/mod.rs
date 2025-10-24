pub mod clock;
mod gallery;
pub mod panel;
pub mod wallpaper;
pub mod windows;

pub use gallery::*;

#[derive(Debug)]
pub enum Feature {
    Clock(clock::Clock),
    Windows(windows::Windows),
}
