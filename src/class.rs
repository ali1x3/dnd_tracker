use iced::widget::{button, column, text};
use iced::{Element, Task};


#[derive(Debug, Default)]
pub struct Class {
    state: State,
}

#[derive(Debug)]
pub enum State {
    Display,
    Edit,
}

impl Default for State {
    fn default() -> Self {
        State::Display
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Display,
    Edit,
}

impl Class{

    pub fn update(&mut self, message: Message) -> Task<Message> {
        let thing = match message {
            Message::Display => self.state = State::Display,
            Message::Edit => self.state = State::Edit,
        }.into();
        thing
    }

    pub fn view(&self) -> Element<'_, Message> {

        let content = match &self.state {
            State::Edit => text("This is editing class"),
            State::Display => text("this is class"),
        };

        column![
            content,
            button(text("edit")).on_press(Message::Edit),
            button(text("display")).on_press(Message::Display),
        ].into()
    }
}

