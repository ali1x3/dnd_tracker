use iced::widget::{button, center, column, container, row, text, Space};
use iced::Length::Fill;
use iced::{border, window, Element, Font, Length, Padding, Task, Theme};

mod class;


#[derive(Default)]
struct App {
    class: class::Class,
    current_screen: Screen,
}

enum Screen {
    Class,
    Home,
}

impl Default for Screen {
    fn default() -> Self {
        Screen::Home
    }
}

#[derive(Debug, Clone)]
enum Message {
    Class(class::Message),
    Home,
    Exit,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Exit => window::get_latest().and_then(window::close),
            Message::Class(msg) => {
                self.current_screen = Screen::Class;
                self.class.update(msg).map(Message::Class)
            },
            Message::Home => {
                self.current_screen = Screen::Home;
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let header = container(row![
            button(text("Characters")).on_press(Message::Home).width(Fill).height(Fill),
            button(text("Races")).on_press(Message::Home).width(Fill).height(Fill),
            button(text("Classes")).on_press(Message::Class(class::Message::Display)).width(Fill).height(Fill),
            button(text("Spells")).on_press(Message::Class(class::Message::Display)).width(Fill).height(Fill),
            button(text("Items")).on_press(Message::Class(class::Message::Display)).width(Fill).height(Fill),
            button(text("Backgrounds")).on_press(Message::Class(class::Message::Display)).width(Fill).height(Fill),
            button(text("Home")).on_press(Message::Home).width(Fill).height(Fill),
            button(text("Exit")).on_press(Message::Exit).width(Fill).height(Fill),
        ]).height(40)
        .padding(Padding {top: 0.0, right: 0.0, bottom: 10.0, left: 0.0} );

        let contents = match self.current_screen {
            Screen::Home => text("this is home").into(),
            Screen::Class => self.class.view().map(Message::Class),
        };

        column![
            header,
            center(contents)
                .style(|theme| {
                    let palette = theme.extended_palette();

                    container::Style::default()
                        .border(border::color(palette.background.strong.color).width(4))
                })
                .padding(20)
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
