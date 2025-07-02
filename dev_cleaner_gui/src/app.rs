use std::path::PathBuf;

use iced::{widget::text, Element, Subscription, Task};

#[derive(Default)]
#[allow(dead_code)]
pub struct App {
    pub title: String,
    pub search_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    None,
}

impl App {
    pub fn new(title: String, search_path: Option<PathBuf>) -> (Self, Task<AppMessage>) {
        (Self { title, search_path },Task::none())
    }

    pub fn subscription(&self) -> Subscription<AppMessage> {
        iced::event::listen_with(|event, _, _id| {
            if let iced::Event::Window(iced::window::Event::CloseRequested) = event {
                // TODO: handle exit Some(AppMessage::Exit(id))
                Some(AppMessage::None)
            } else {
                None
            }
        })
    }

    pub fn update(&mut self, _message: AppMessage) -> Task<AppMessage> {
        Task::none()
    }


    pub fn view(&self, _id: iced::window::Id) -> Element<'_, AppMessage> {
        text("Hello, world!").into()
    }
}
