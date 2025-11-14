use iced::widget::{button, column, row, text, Row};
use iced::{Element, Font, Task, Theme, window};

mod class;


#[derive(Default)]
struct App {
    class: class::Class,
}

#[derive(Debug, Clone)]
enum Message {
    Class(class::Message),
    Exit,
}

impl App {

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::Class(msg) => self.class.update(msg).map(Message::Class),
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = row![
            text("app").size(20).font(Font::MONOSPACE),
            button(text("Exit")).on_press(Message::Exit),
        ];


        column![
            header,
            button(text("Classes")).on_press(Message::Class(class::Message::Display)),
        ].into()
    }

    fn theme(&self) -> Theme {
        Theme::CatppuccinMocha
    }
}

fn main() -> iced::Result {
    iced::application("D&D Tracker", App::update, App::view)
        .theme(App::theme)
        .run()
}
