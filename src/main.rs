use iced::widget::{button, column, row, text, Row};
use iced::{window, Element, Task};

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

fn update(app: &mut App, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::Class(msg) => app.class.update(msg).map(Message::Class),
    }
}

fn view(_app: &App) -> Element<Message> {
    let header = row![
        text("app"),
        button(text("Exit")).on_press(Message::Exit),
    ];
    column![
        button(text("Classes")).on_press(Message::Class(class::Message::Display)),
    ].into()
}

fn title(app: &App) -> String {
    String::from("D&D Tracker")
}

fn main() {
    let _ = iced::run("a cool counter", update, view);
}
