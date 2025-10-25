use std::time::Duration;

use chrono::{DateTime, Local};
use iced::{Element, Subscription, Task, time};

use crate::feature::clock::Message;

#[derive(Debug)]
pub struct Clock {
    now: DateTime<Local>,
    freq: Duration,
}

impl Clock {
    pub fn new(freq: Duration) -> Self {
        let now = Local::now();

        Self { now, freq }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Tick => {
                self.now = Local::now();
            }
        }

        Task::none()
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        time::every(self.freq).map(|_| Message::Tick)
    }
}
