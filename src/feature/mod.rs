pub mod battery;
pub mod clock;
pub mod panels;
pub mod wallpaper;

pub use clock::Clock;
pub use panels::Panels;

#[derive(Debug)]
pub enum Feature {
    Clock(Clock),
    Panels(Panels),
}
