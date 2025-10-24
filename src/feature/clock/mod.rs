mod action;
mod message;

use std::time::Duration;

pub use action::*;
use chrono::{DateTime, Local};
use iced::{Element, Subscription, Task, time, widget};
pub use message::*;

#[derive(Debug)]
pub struct Clock {
    now: DateTime<Local>,
    precision: Duration,
}

impl Clock {
    pub fn new(precision: Duration) -> Self {
        let now = Local::now();

        Self { now, precision }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Set(dt) => {
                self.now = dt;
            }
        }

        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        widget::column![widget::text!["{:?}", self.now]].into()
    }

    pub fn subscription(&self) -> Subscription<Action> {
        time::every(self.precision).map(|_| Action::Run(Self::set_clock()))
    }

    fn set_clock() -> Task<Message> {
        Task::future(async { Message::Set(Local::now()) })
    }
}
