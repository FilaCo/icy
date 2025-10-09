use std::path::PathBuf;

use clap::Parser;
use iced::{Element, Task, window};

use crate::{cli::Cli, message::FlcsMessage};

#[derive(Debug)]
pub struct Flcs {}

impl Flcs {
    pub fn new() -> (Self, Task<FlcsMessage>) {
        let args = Cli::parse();
        todo!()
    }

    pub fn update(&mut self, msg: FlcsMessage) -> Task<FlcsMessage> {
        todo!()
    }

    pub fn view(&self, _: window::Id) -> Element<FlcsMessage> {
        todo!()
    }

    pub fn title(&self, _: window::Id) -> String {
        "flcs".to_string()
    }
}
