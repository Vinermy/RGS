use crate::gui::Rgs;
use iced;
use iced::{Application, Settings};


mod gui;
pub mod body;
pub mod vec2d;
mod message;
pub mod frame;

fn main() -> iced::Result {
    Rgs::run(Settings::default())
}
