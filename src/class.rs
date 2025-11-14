use iced::widget::{button, text};
use iced::{Element, Task};


#[derive(Debug, Default)]
pub struct Class {
}

#[derive(Debug, Clone)]
pub enum Message {
    Display,
    Exit,
}

impl Class{
    pub fn update(&mut self, message: Message) -> Task<Message> {
        Task::none()
    }

    pub fn view(&self) -> Element<'_, Message> {
        text("class").into()
    }
}

