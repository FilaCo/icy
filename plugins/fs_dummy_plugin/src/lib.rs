use extism_pdk::plugin_fn;
use fs_plugin::message::Message;
use iced::{Element, Renderer, Theme, core::Widget};

#[plugin_fn]
pub fn view() -> impl Widget<Message, Theme, Renderer> {
    todo!()
}
