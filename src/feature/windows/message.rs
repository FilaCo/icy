use iced::window;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Open,
    Opened(window::Id),
}
