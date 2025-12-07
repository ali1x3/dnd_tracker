use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc;

use anyhow::Error;
use eframe::egui::panel::TopBottomSide;
use eframe::egui::{layers, Button, Layout, RichText};
use eframe::egui::{self, CentralPanel, SidePanel, TopBottomPanel, ViewportBuilder};

mod consts;
mod db;
mod pages;

use eframe::epaint::MarginF32;
use pages::{classes, races};

use crate::pages::{weapons, Pages};
use db::models;

//controller
struct App {
    screen: Screen,
    command_sender: mpsc::Sender<Command>, //database
    response_receiver: mpsc::Receiver<DbResponse>,
    class_page: classes::Class, //views
    race_page: races::Race,
    weapon_page: weapons::Weapon,
    model: Rc<RefCell<Model>>, //model
}

#[derive(Clone)]
struct Model {
    classes: Option<Vec<models::RacialTrait>>,
    selected_class: Option<models::RacialTrait>,
    races: Option<Vec<models::Race>>,
    selected_race: Option<models::Race>,
    weapons: Option<Vec<models::Weapon>>,
    selected_weapon: Option<models::Weapon>,
    label: Option<String>,
}

impl Model{
    pub fn new() -> Self {
        Self {
            classes: None,
            selected_class: None,
            races: None,
            selected_race: None,
            weapons: None,
            selected_weapon: None,
            label: None,
        }
    }
}

enum DbResponse {
    Races(Option<Vec<models::Race>>),
    Weapons(Option<Vec<models::Weapon>>),
    Label(Option<String>),
}

enum Command {
    LoadRaces,
    CreateRace(models::Race),
    DeleteRace(surrealdb::sql::Thing),
    LoadWeapons,
    CreateWeapon(models::Weapon),
    DeleteWeapon(surrealdb::sql::Thing),
}

enum Screen {
    Home,
    Characters,
    Races,
    Classes,
    Spells,
    Weapons,
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
                if ui.selectable_value(&mut self.screen, Screen::Weapons, nav_button_builder("Weapons")).clicked() {
                    let _ = self.command_sender.send(Command::LoadWeapons);
                };
                ui.selectable_value(&mut self.screen, Screen::Backgrounds, nav_button_builder("Backgrounds"));
                ui.selectable_value(&mut self.screen, Screen::Exit, nav_button_builder("Exit"));
            });
        });

        SidePanel::left("test") .default_width(250.0).width_range(200.0..=500.0).show(ctx, |ui| {
            if let Ok(response) = self.response_receiver.try_recv() {
                match response {
                    DbResponse::Races(races) => {
                        self.model.borrow_mut().races = races;
                    },
                    DbResponse::Label(label) => {
                        self.model.borrow_mut().label = label;
                    },
                    DbResponse::Weapons(weapons) => {
                        self.model.borrow_mut().weapons = weapons;
                    },
                }
            }

            match self.screen {
                Screen::Home => {},
                Screen::Characters => {},
                Screen::Races => {
                    let races_list = self.model.borrow().races.clone();
                    match races_list {
                        Some(races) => {
                            for race in races {
                                egui::Frame::default().inner_margin(MarginF32::symmetric(5.0, 3.0)).show(ui, |ui| {
                                    let text = RichText::new(&race.race_name).size(20.0);
                                    let button = Button::new(text);
                                    let size = [ui.available_width(), 38.0];
                                    let button = ui.add_sized(size, button);
                                    if button.clicked() {
                                        self.model.borrow_mut().selected_race = Some(race);
                                    };
                                });
                            }
                            egui::Frame::default().inner_margin(MarginF32::symmetric(5.0, 3.0)).show(ui, |ui| {
                                let text = RichText::new("+").size(20.0);
                                let button = Button::new(text);
                                let size = [ui.available_width(), 38.0];
                                let button = ui.add_sized(size, button);
                                if button.clicked() {
                                    self.race_page.state = races::State::Create;
                                };
                            });
                        },
                        None => {
                            ui.spinner();
                        },
                    }
                },
                Screen::Classes => {},
                Screen::Spells => {},
                Screen::Weapons => {
                    let weapons_list = self.model.borrow().weapons.clone();
                    match weapons_list {
                        Some(weapons) => {
                            for weapon in weapons {
                                egui::Frame::default().inner_margin(MarginF32::symmetric(5.0, 3.0)).show(ui, |ui| {
                                    let text = RichText::new(&weapon.weapon_name).size(20.0);
                                    let button = Button::new(text);
                                    let size = [ui.available_width(), 38.0];
                                    let button = ui.add_sized(size, button);
                                    if button.clicked() {
                                        self.model.borrow_mut().selected_weapon = Some(weapon);
                                    };
                                });
                            }
                            egui::Frame::default().inner_margin(MarginF32::symmetric(5.0, 3.0)).show(ui, |ui| {
                                let text = RichText::new("+").size(20.0);
                                let button = Button::new(text);
                                let size = [ui.available_width(), 38.0];
                                let button = ui.add_sized(size, button);
                                if button.clicked() {
                                    let test = models::Weapon {
                                        id: surrealdb::sql::Thing::from((consts::WEAPON_TABLE, surrealdb::sql::Id::rand())),
                                        weapon_name: "test".into(),
                                        damage_dice: vec![models::Die::D8, models::Die::D8, models::Die::D6],
                                        weight: 2.0,
                                        description: "descriptoion herer".into(),
                                        properties: vec!["one".into(), "two".into()],
                                        damage_type: "idk something".into(),
                                    };
                                    self.weapon_page.state = weapons::State::Create;
                                    self.command_sender.send(Command::CreateWeapon(test));
                                };
                            });
                        },
                        None => {
                            ui.spinner();
                        },
                    }
                },
                Screen::Backgrounds => {},
                Screen::Exit => {},
            }
        });

        CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                Screen::Home => {
                    ui.heading("this is home, not yet implemented. Go to Race");
                }
                Screen::Classes => {
                    self.class_page.update(ctx, ui, frame);
                }
                Screen::Characters => {
                    ui.heading("this is characters, not yet implemented. Go to Race");
                }
                Screen::Races => {
                    self.race_page.update(ctx, ui, frame);
                }
                Screen::Spells => {
                    ui.heading("this is spells, not yet implemented. Go to Race");
                }
                Screen::Weapons => {
                    self.weapon_page.update(ctx, ui, frame);
                }
                Screen::Backgrounds => {
                    ui.heading("this is backgrounds, not yet implemented. Go to Race");
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

    let model = Rc::new(RefCell::new(Model::new()));

    let app = App {
        screen: Screen::Home,
        command_sender: command_sender.clone(),
        response_receiver: response_reciever,
        class_page: classes::Class {},
        race_page: races::Race::new(Rc::clone(&model), command_sender.clone()),
        weapon_page: weapons::Weapon::new(Rc::clone(&model), command_sender.clone()),
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
