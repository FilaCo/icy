use iced_sessionlock::to_session_message;

#[to_session_message]
#[derive(Debug, Clone)]
pub enum LockMessage {}
