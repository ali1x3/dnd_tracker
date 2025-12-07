use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{frame, Align, Button, Color32, Layout, RichText, Stroke, Ui}, epaint::MarginF32};

use super::*;
use crate::{consts, db::models::{self, AbilityScoreBonus, BuildError, RaceBuilder, RacialTrait}, Model};

pub struct Race {
    model: Rc<RefCell<Model>>,
    command_sender: Sender<Command>,
    pub state: State,
    builder: RaceBuilder,
    temp: TempRace,
    build_status: Result<(), BuildError>,
}

pub struct TempRace {
    race_name: String,
    move_speed: String,
    strength: Counter,
    dexterity: Counter,
    constitution: Counter,
    intelligence: Counter,
    wisdom: Counter,
    charisma: Counter,
    languages: Vec<String>,
    skills: Vec<models::Skill>,
    traits: Vec<models::RacialTrait>,
    new_language_input: String,
    trait_name: String,
    trait_description: String,
    trait_level: String,
}

impl TempRace {
    fn clear(&mut self) {
        self.race_name.clear();
        self.move_speed.clear();
        self.strength.counter = 0;
        self.dexterity.counter = 0;
        self.constitution.counter = 0;
        self.intelligence.counter = 0;
        self.wisdom.counter = 0;
        self.charisma.counter = 0;
        self.languages.clear();
        self.skills.clear();
        self.traits.clear();
        self.new_language_input.clear();
        self.trait_name.clear();
        self.trait_level.clear();
        self.trait_description.clear();
    }
}

pub enum State {
    View,
    Create,
    ConfirmCreate,
    ErrorCreate,
    Delete,
}


impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Race {
    pub fn new(model: Rc<RefCell<Model>>, command_sender: Sender<Command>) -> Self {
        Race {
            model,
            command_sender,
            state: State::View,
            builder: RaceBuilder::new(),
            build_status: Ok(()),
            temp: TempRace {
                race_name: String::new(),
                move_speed: String::from("30"),
                strength: Counter::new("STR"),
                dexterity: Counter::new("DEX"),
                constitution: Counter::new("CON"),
                intelligence: Counter::new("INT"),
                wisdom: Counter::new("WIS"),
                charisma: Counter::new("CHA"),
                languages: Vec::new(),
                skills: Vec::new(),
                traits: Vec::new(),
                trait_description: String::new(),
                new_language_input: String::new(),
                trait_name: String::new(),
                trait_level: String::new(),
            } 
        }
    }
}

struct Counter {
    name: String,
    counter: i8,
}

impl Counter {
    fn new(name: &str) -> Self {
        Counter { name: name.to_string(), counter: 0 }
    }

    fn view(&mut self, ui: &mut Ui) {
        make_frame(MarginF32::same(0.0)).show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.heading(text_builder(&self.name, 25.0).strong());
                ui.heading(text_builder(&self.counter.to_string(), 25.0).strong());
                ui.horizontal(|ui| {
                    ui.add_space(ui.available_width() * 0.3);
                    let text = RichText::new("-").size(20.0);
                    let button = Button::new(text);
                    let size = [25.0, 25.0];
                    let button = ui.add_sized(size, button);
                    if button.clicked(){
                        self.counter -= 1;
                    };

                    ui.add_space(0.5);

                    let text = RichText::new("+").size(20.0);
                    let button = Button::new(text);
                    let size = [25.0, 25.0];
                    let button = ui.add_sized(size, button);
                    if button.clicked(){
                        self.counter += 1;
                    };
                });
            });
        });
    }
}

impl Pages for Race {
    fn update(&mut self, ctx: &egui::Context, ui: &mut Ui, frame: &mut eframe::Frame) {
        let selected_race = &self.model.borrow().clone().selected_race;
        match selected_race {
            Some(race) => {
                make_frame(MarginF32{ left: 30.0, right: 0.0, top: 20.0, bottom: 20.0 }).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(text_builder(&race.race_name, 30.0).strong());
                        ui.add_space(ui.available_width() * 0.8);

                        let text = RichText::new("Edit").size(20.0);
                        let button = Button::new(text);
                        let size = [70.0, 30.0];
                        let button = ui.add_sized(size, button);
                        if button.clicked(){
                        };

                        let text = RichText::new("Delete").size(20.0);
                        let button = Button::new(text);
                        let size = [70.0, 30.0];
                        let button = ui.add_sized(size, button);
                        if button.clicked(){
                            self.state = State::Delete;
                        };
                    });
                });

                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        ui.heading(text_builder("Ability Score Bonus:", 25.0).strong());
                    });
                });

                make_frame(MarginF32{ left: 30.0, right: 30.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.columns(6, |columns| {
                        columns[0].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("STR", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.strength.to_string(), 25.0));
                        });
                        columns[1].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("DEX", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.dexterity.to_string(), 25.0));
                        });
                        columns[2].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("CONS", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.constitution.to_string(), 25.0));
                        });
                        columns[3].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("INT", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.intelligence.to_string(), 25.0));
                        });
                        columns[4].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("WIS", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.wisdom.to_string(), 25.0));
                        });
                        columns[5].with_layout(Layout::top_down(egui::Align::Center), |ui| {
                            ui.heading(text_builder("CHA", 25.0).strong());
                            ui.heading(text_builder(&race.ability_score_increase.charisma.to_string(), 25.0));
                        });
                    });
                });
                ui.add_space(15.0);

                ui.columns(2, |columns| {
                    make_frame(MarginF32::symmetric(30.0, 10.0)).show(&mut columns[0], |ui| {
                        ui.with_layout(Layout::default(), |ui| {
                            ui.heading(text_builder("Racial Traits: ", 25.0).strong());
                            for race_trait in &race.racial_traits {
                                let text = "  - [".to_owned() + &race_trait.level.to_string() + "] " + &race_trait.trait_name;
                                ui.label(text_builder(&text, 20.0).strong()).on_hover_ui(|ui| {
                                    ui.label(text_builder(&race_trait.description, 20.0).strong());
                                });
                            }

                            ui.add_space(20.0);

                            ui.heading(text_builder("Languages: ", 25.0).strong());
                            for language in &race.languages {
                                let text = "  - ".to_owned() + language;
                                ui.label(text_builder(&text, 20.0).strong());
                            }
                        });
                    });

                    make_frame(MarginF32::symmetric(30.0, 10.0)).show(&mut columns[1], |ui| {
                        ui.with_layout(Layout::default(), |ui| {
                            ui.heading(text_builder("Skill Proficencies: ", 25.0).strong());
                            for skill in &race.skill_proficiency_bonus {
                                let text = "  - ".to_owned() + &skill.to_string();
                                ui.label(text_builder(&text, 20.0).strong());
                            }

                            ui.add_space(20.0);
                            ui.heading(text_builder("Move Speed: ", 25.0).strong());
                            let text = "  - ".to_owned() + &race.move_speed.to_string() + " ft.";
                            ui.heading(text_builder(&text, 20.0).strong());
                        });
                    });
                });
            }
            None => {
                ui.heading("click on a race!");
            }
        }
        if self.state == State::Delete {
            match show_confirmation_dialog(ctx, "Delete Race?", "Are you sure you want to delete?") {
                ConfirmationResult::Pending => {},
                ConfirmationResult::Confirmed => {
                    self.state = State::View;

                    self.temp.clear();
                    let selected_race = self.model.borrow_mut().selected_race.clone();
                    self.command_sender.send(Command::DeleteRace(selected_race.unwrap().id));
                    self.model.borrow_mut().selected_race = None;
                    self.command_sender.send(Command::LoadRaces);
                },
                ConfirmationResult::Cancelled => {
                    self.state = State::View;
                },
            }
        }

        if self.state == State::Create || self.state == State::ConfirmCreate || self.state == State::ErrorCreate {
            egui::Window::new("Create Race").resizable([false,false]).show(ctx, |ui| {
                let size = [900.0, 700.0];
                ui.set_min_size(size.into());
                ui.set_max_size(size.into());
                ui.horizontal(|ui| {
                    ui.heading("");
                    ui.add_space(ui.available_width());

                    let text = RichText::new("Confirm").size(20.0);
                    let button = Button::new(text);
                    let size = [90.0, 30.0];
                    let button = ui.add_sized(size, button);
                    if button.clicked(){
                        self.state = State::ConfirmCreate;
                    };

                    let text = RichText::new("Exit").size(20.0);
                    let button = Button::new(text);
                    let size = [70.0, 30.0];
                    let button = ui.add_sized(size, button);
                    if button.clicked(){
                        self.temp.clear();
                        self.state = State::View;
                    };
                });


                ui.horizontal(|ui| {
                    ui.heading("Race Name: ");
                    let textfield = ui.text_edit_singleline(&mut self.temp.race_name);
                });

                ui.add_space(10.0);
                ui.columns(6, |columns| {
                    self.temp.strength.view(&mut columns[0]);
                    self.temp.dexterity.view(&mut columns[1]);
                    self.temp.constitution.view(&mut columns[2]);
                    self.temp.intelligence.view(&mut columns[3]);
                    self.temp.wisdom.view(&mut columns[4]);
                    self.temp.charisma.view(&mut columns[5]);
                });

                ui.add_space(20.0);
                ui.columns(2, |columns| {
                    columns[0].horizontal(|ui| {
                        ui.heading("Racial Trait Name: ");
                        let textfield = ui.text_edit_singleline(&mut self.temp.trait_name);
                        if textfield.changed() {
                            println!("{}", self.temp.race_name);
                        }
                    });

                    columns[0].horizontal(|ui| {
                        ui.heading("Racial Trait Level: ");
                        let textfield = ui.text_edit_singleline(&mut self.temp.trait_level);
                        if textfield.changed() {
                            println!("{}", self.temp.race_name);
                        }
                    });

                    columns[0].horizontal(|ui| {
                        ui.heading("Racial Trait Description: ");
                        let textfield = ui.text_edit_multiline(&mut self.temp.trait_description);
                        if textfield.changed() {
                            println!("{}", self.temp.race_name);
                        }
                    });
                    columns[0].horizontal(|ui| {
                        let text = RichText::new("add").size(20.0);
                        let button = Button::new(text);
                        let size = [70.0, 30.0];
                        if ui.add_sized(size, button).clicked(){
                            if !(self.temp.trait_name.is_empty() || self.temp.trait_description.is_empty()){
                                match self.temp.trait_level.parse::<i8>() {
                                    Ok(level) => {
                                        self.temp.traits.push(RacialTrait{trait_name: self.temp.trait_name.clone(), description: self.temp.trait_description.clone(), level: level});
                                        self.temp.traits.sort_by_key(|race| race.level);
                                        self.temp.trait_name.clear();
                                        self.temp.trait_level.clear();
                                        self.temp.trait_description.clear();
                                    },
                                    Err(_) =>  {},
                                };
                            }
                        }
                        ui.add_space(ui.available_width()); // Push everything else to the right

                    });
                    columns[0].heading(text_builder("Racial Traits: ", 20.0).strong());
                    for race_trait in &self.temp.traits {
                        let text = "  - [".to_owned() + &race_trait.level.to_string() + "] " + &race_trait.trait_name;

                        columns[0].label(text_builder(&text, 20.0).strong()).on_hover_ui(|ui| {
                            ui.label(text_builder(&race_trait.description, 20.0).strong());
                        });
                    }

                    columns[0].horizontal(|ui| {
                        ui.heading("Language: ");
                        let textfield = ui.text_edit_singleline(&mut self.temp.new_language_input);
                        if textfield.changed() {
                            println!("{}", self.temp.new_language_input);
                        }
                    });

                    columns[0].add_space(20.0);
                    columns[0].heading(text_builder("Languages: ", 20.0).strong());

                    columns[0].horizontal(|ui| {
                        let text = RichText::new("add").size(20.0);
                        let button = Button::new(text);
                        let size = [70.0, 30.0];
                        if ui.add_sized(size, button).clicked(){
                            if !self.temp.new_language_input.is_empty(){
                                self.temp.languages.push(self.temp.new_language_input.clone());
                                self.temp.new_language_input.clear();
                            }
                        }
                        ui.add_space(ui.available_width()); // Push everything else to the right

                    });

                    for language in &self.temp.languages {
                        let text = "  - ".to_owned() + &language;
                        columns[0].label(text_builder(&text, 20.0).strong());
                    }


                    columns[0].horizontal(|ui| {
                        ui.heading("Move Speed: ");
                        let textfield = ui.text_edit_singleline(&mut self.temp.move_speed);
                        if textfield.changed() {
                            println!("{}", self.temp.new_language_input);
                        }
                    });


                    columns[1].heading(text_builder("Skill Proficiencies: ", 20.0).strong());
                    columns[1].columns(2, |columns| {
                        let skills = vec![
                            models::Skill::Acrobatics,
                            models::Skill::AnimalHandling,
                            models::Skill::Arcana,
                            models::Skill::Athletics,
                            models::Skill::Deception,
                            models::Skill::History,
                            models::Skill::Insight,
                            models::Skill::Intimidation,
                            models::Skill::Investigation,
                            models::Skill::Medicine,
                            models::Skill::Nature,
                            models::Skill::Perception,
                            models::Skill::Performance,
                            models::Skill::Persuasion,
                            models::Skill::Religion,
                            models::Skill::SleightOfHand,
                            models::Skill::Stealth,
                            models::Skill::Survival,
                        ];

                        let mid = skills.len() / 2;

                        // Left column
                        for skill in &skills[..mid] {
                            let mut is_selected = self.temp.skills.contains(skill);
                            if columns[0].checkbox(&mut is_selected, skill.to_string()).clicked() {
                                if is_selected {
                                    self.temp.skills.push(skill.clone());
                                } else {
                                    self.temp.skills.retain(|s| s != skill);
                                }
                            }
                        }

                        // Right column
                        for skill in &skills[mid..] {
                            let mut is_selected = self.temp.skills.contains(skill);
                            if columns[1].checkbox(&mut is_selected, skill.to_string()).clicked() {
                                if is_selected {
                                    self.temp.skills.push(skill.clone());
                                } else {
                                    self.temp.skills.retain(|s| s != skill);
                                }
                            }
                        }
                        columns[1].add_space(columns[1].available_height());
                    });
                });
            });
            if self.state == State::ConfirmCreate {
                match show_confirmation_dialog(ctx, "", "") {
                    ConfirmationResult::Pending => {},
                    ConfirmationResult::Confirmed => {
                        let mut builder = self.builder.clone();
                        let move_speed = self.temp.move_speed.clone().parse::<i16>().unwrap_or_default();
                        builder
                            .id(consts::RACE_TABLE)
                            .race_name(self.temp.race_name.clone())
                            .move_speed(move_speed)
                            .ability_score_increase(AbilityScoreBonus {
                                strength: self.temp.strength.counter,
                                dexterity: self.temp.dexterity.counter,
                                constitution: self.temp.constitution.counter,
                                intelligence: self.temp.intelligence.counter,
                                wisdom: self.temp.wisdom.counter,
                                charisma: self.temp.charisma.counter,
                            });
                        for racial_trait in self.temp.traits.clone() {
                            builder.add_racial_trait(racial_trait);
                        }

                        for language in self.temp.languages.clone() {
                            builder.add_language(language);
                        }

                        for skill in self.temp.skills.clone() {
                            builder.add_skill_proficiency(skill);
                        }

                        let race = match builder.build() {
                            Ok(race) => {
                                let _ = self.command_sender.send(Command::CreateRace(race));
                                self.temp.skills.clear();
                                self.temp.languages.clear();
                                self.temp.race_name.clear();
                                self.temp.new_language_input.clear();
                                self.temp.trait_description.clear();
                                self.temp.trait_name.clear();
                                self.temp.trait_level.clear();
                                self.temp.traits.clear();
                                self.state = State::View;
                            },
                            Err(err) => {
                                self.state = State::ErrorCreate;
                                self.build_status = Err(err);
                            },
                        };
                        println!("{:#?}", race);
                        let _ = self.command_sender.send(Command::LoadRaces);
                    },
                    ConfirmationResult::Cancelled => {
                        self.state = State::Create;
                    },
                };
            }
            if self.state == State::ErrorCreate {
                let build_status = self.build_status.clone();
                if show_info_dialog(ctx, "Race Create Error", &build_status.err().unwrap().to_string(), true) {
                    self.state = State::Create;
                }
            }
        }
    }
}

fn text_builder(text: &str, size: f32) -> RichText {
    RichText::new(text).size(size)
}

fn make_border() -> Stroke {
    Stroke::new(1.0, Color32::RED)
}

fn make_frame(margin: MarginF32) -> egui::Frame {
    egui::Frame::default().inner_margin(margin)
}

fn show_confirmation_dialog(
    ctx: &egui::Context,
    title: &str,
    message: &str,
) -> ConfirmationResult {
    // Draw backdrop
    
    let mut result = ConfirmationResult::Pending;
    
    // Handle ESC key
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        return ConfirmationResult::Cancelled;
    }
    
    egui::Window::new(format!("⚠ {}", title))
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(350.0);
            
            ui.label(message);
            ui.add_space(5.0);
            ui.colored_label(Color32::RED, "⚠ This action cannot be undone.");
            ui.add_space(15.0);
            
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() / 2.0 - 85.0);
                
                if ui.add_sized([80.0, 30.0], Button::new("Cancel")).clicked() {
                    result = ConfirmationResult::Cancelled;
                }
                
                if ui.add_sized([80.0, 30.0], 
                    Button::new(RichText::new("Confirm").color(Color32::WHITE))
                        .fill(Color32::from_rgb(180, 0, 0))
                ).clicked() {
                    result = ConfirmationResult::Confirmed;
                }
            });
        });
    
    result
}

fn show_info_dialog(
    ctx: &egui::Context,
    title: &str,
    message: &str,
    is_error: bool,
) -> bool {
    let mut should_close = false;
    
    // Handle ESC key
    if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        return true;
    }
    
    let window_title = if is_error {
        format!("⚠ {}", title)
    } else {
        format!("ℹ {}", title)
    };
    
    egui::Window::new(window_title)
        .collapsible(false)
        .resizable(false)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.set_min_width(350.0);
            
            let color = if is_error { Color32::RED } else { Color32::WHITE };
            ui.colored_label(color, message.to_string());
            
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() / 2.0 - 35.0);
                
                if ui.add_sized([80.0, 30.0], Button::new("Cancel")).clicked() {
                    should_close = true;
                }
            });
        });
    
    should_close
}

pub enum ConfirmationResult {
    Pending,
    Confirmed,
    Cancelled,
}
