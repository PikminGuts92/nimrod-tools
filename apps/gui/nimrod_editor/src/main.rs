// Hide console if release build
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::alignment::{self, Alignment};
use iced::executor;
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, radio, row,
    scrollable, slider, text, text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Application, Color, Command, Element, Font, Length, Pixels, Sandbox, Settings, Theme};


pub fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    NimrodEditor::run(Settings::default())
}

pub struct NimrodEditor {}

impl Application for NimrodEditor {
    type Executor = executor::Default;
    type Message = ();
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("Nimrod Editor")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        "Hey, nimrod!".into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dracula
    }
}