pub mod clock;
pub mod panel;
pub mod wallpaper;

pub use clock::Clock;

#[derive(Debug)]
pub enum Feature {
    Clock(Clock),
}
