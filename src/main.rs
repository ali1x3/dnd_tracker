use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use anyhow::Error;
use eframe::egui::{layers, RichText};
use eframe::egui::{self, CentralPanel, SidePanel, TopBottomPanel, ViewportBuilder};

mod consts;
mod db;
mod pages;

use pages::{classes, races};

use crate::pages::Pages;
use db::models;

//controller
struct App {
    screen: Screen,
    command_sender: mpsc::Sender<Command>, //database
    response_receiver: mpsc::Receiver<DbResponse>,
    class_page: classes::Class, //views
    race_page: races::Race,
    model: Rc<RefCell<Model>>, //model
}

#[derive(Clone)]
struct Model {
    classes: Option<Vec<models::RacialTrait>>,
    selected_class: Option<models::RacialTrait>,
    label: Option<String>,
}

enum DbResponse {
    Races(Option<Vec<models::RacialTrait>>),
    Label(Option<String>),
}

enum Command {
    LoadRaces,
}

enum Screen {
    Home,
    Characters,
    Races,
    Classes,
    Spells,
    Items,
    Backgrounds,
    Exit,
}

impl PartialEq for Screen {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        TopBottomPanel::top("top") .exact_height(28.0) .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.screen, Screen::Home, nav_button_builder("Home"));
                ui.selectable_value(&mut self.screen, Screen::Characters, nav_button_builder("Characters"));
                ui.selectable_value(&mut self.screen, Screen::Classes, nav_button_builder("Class"));
                if ui.selectable_value(&mut self.screen, Screen::Races, nav_button_builder("Races")).clicked() {
                    let _ = self.command_sender.send(Command::LoadRaces);
                };
                ui.selectable_value(&mut self.screen, Screen::Spells, nav_button_builder("Spells"));
                ui.selectable_value(&mut self.screen, Screen::Items, nav_button_builder("Items"));
                ui.selectable_value(&mut self.screen, Screen::Backgrounds, nav_button_builder("Backgrounds"));
                ui.selectable_value(&mut self.screen, Screen::Exit, nav_button_builder("Exit"));
            });
        });

        SidePanel::left("test") .default_width(250.0).width_range(200.0..=500.0).show(ctx, |ui| {
            if let Ok(response) = self.response_receiver.try_recv() {
                match response {
                    DbResponse::Races(racial_traits) => {
                        self.model.borrow_mut().classes = racial_traits;
                    },
                    DbResponse::Label(label) => {
                        self.model.borrow_mut().label = label;
                    },
                }
            }
            let binding = self.model.borrow().clone();
            let label = match &binding.label  {
                Some(label) => label,
                None => "Noghint",
            };

            let races_list = self.model.borrow().classes.clone();
            match races_list {
                Some(races) => {
                    for race in races {
                        if ui.button(&race.trait_name).clicked() {
                            self.model.borrow_mut().selected_class = Some(race);
                        };
                    }
                },
                None => {
                    ui.heading("no races!");
                    ui.spinner();
                },
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screen::Home => {
                    ui.heading("this is home");
                }
                Screen::Classes => {
                    self.class_page.update(ctx, ui, frame);
                }
                Screen::Characters => {
                    ui.heading("this is characters");
                }
                Screen::Races => {
                    self.race_page.update(ctx, ui, frame);
                }
                Screen::Spells => {
                    ui.heading("this is spells");
                }
                Screen::Items => {
                    ui.heading("this is items");
                }
                Screen::Backgrounds => {
                    ui.heading("this is backgrounds");
                }
                Screen::Exit => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            };
        });

        ctx.request_repaint();
    }
}

fn main() -> Result<(), Error> {
    let (command_sender, command_reciever) = mpsc::channel();
    let (response_sender, response_reciever) = mpsc::channel();

    std::thread::spawn(|| -> Result<(), Error> {
        let rt = tokio::runtime::Runtime::new()?;

        let conn_status: Result<(), Error> =
        rt.block_on(db::init_db(command_reciever, response_sender));

        match conn_status {
            Ok(_) => println!("db connec"),
            Err(_) => println!("db no connec"),
        };

        Ok(())
    });

    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title("D&D Character Tracker")
            .with_inner_size([1200.0, 900.0])
            .with_min_inner_size([1200.0, 900.0]),
        ..Default::default()
    };

    let model = Rc::new(RefCell::new(Model {
        classes: None,
        selected_class: None,
        label: None,
    }));

    let app = App {
        screen: Screen::Home,
        command_sender: command_sender.clone(),
        response_receiver: response_reciever,
        class_page: classes::Class {},
        race_page: races::Race::new(Rc::clone(&model), command_sender.clone()),
        model: model,
    };

    let _ = eframe::run_native(
        "D&D Character Tracker",
        options,
        Box::new(|_| Ok(Box::new(app))),
    );
    Ok(())
}

fn nav_button_builder(text: &str) -> RichText {
    RichText::new(text).size(consts::NAV_BUTTONS_SIZE)
}
