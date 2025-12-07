use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{Button, Color32, RichText, Stroke}, epaint::MarginF32};

use crate::{db::models::{self, Die}, Model};

use super::*;

pub struct Weapon {
    model: Rc<RefCell<Model>>,
    command_sender: Sender<Command>,
    pub state: State,
    temp: TempWeapon,
}

struct TempWeapon {
    weapon_name: String,
    damage_dice: Vec<Die>,
    weight: f32,
    description: String,
    properties: Vec<String>,
    damage_type: String,
}

impl TempWeapon {
    fn clear(&mut self) {
        self.weapon_name.clear();
        self.damage_dice.clear();
        self.weight = 0.0;
        self.description.clear();
        self.properties.clear();
        self.damage_type.clear();
    }
}

impl Weapon {
    pub fn new(model: Rc<RefCell<Model>>, command_sender: Sender<Command>) -> Self {
        Self {
            model,
            command_sender,
            state: State::View,
            temp: TempWeapon {
                weapon_name: String::new(),
                damage_dice: vec![],
                weight: 0.0,
                description: String::new(),
                properties: vec![],
                damage_type: String::new(),
            }
        }
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

impl Pages for Weapon {
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let selected_weapon = &self.model.borrow().clone().selected_weapon;
        match selected_weapon {
            Some(weapon) => {
                make_frame(MarginF32{ left: 30.0, right: 0.0, top: 20.0, bottom: 20.0 }).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(text_builder(&weapon.weapon_name, 30.0).strong());
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
                    let text = "Description: ";
                    ui.heading(text_builder(&text, 25.0).strong());
                    ui.heading(text_builder(&weapon.description, 25.0).strong());
                });

                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        let text = "Damage Type: ".to_string() + &weapon.damage_type;
                        ui.heading(text_builder(&text, 25.0).strong());
                    });
                });

                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    let text = "Properties: ";
                    ui.heading(text_builder(&text, 25.0).strong());

                    for property in &weapon.properties {
                        let text = "   - ".to_string() + property;
                        ui.heading(text_builder(&text, 25.0).strong());
                    }
                });

                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                        let text = "Damage Die: ";
                        ui.heading(text_builder(&text, 25.0).strong());
                        ui.heading(text_builder(&format_damage_dice(&weapon.damage_dice), 25.0));
                    });
                });
            },
            None => {
                ui.heading("click on a weapon!");
            },
        }


        if self.state == State::Delete {
            match show_confirmation_dialog(ctx, "Delete Race?", "Are you sure you want to delete?") {
                ConfirmationResult::Pending => {},
                ConfirmationResult::Confirmed => {
                    self.state = State::View;

                    //self.temp.clear();
                    let selected_weapon = self.model.borrow_mut().selected_weapon.clone();
                    self.command_sender.send(Command::DeleteWeapon(selected_weapon.unwrap().id));
                    self.model.borrow_mut().selected_race = None;
                    self.command_sender.send(Command::LoadWeapons);
                },
                ConfirmationResult::Cancelled => {
                    self.state = State::View;
                },
            }
        }

        if self.state == State::Create || self.state == State::ConfirmCreate || self.state == State::ErrorCreate {
            egui::Window::new("Create Weapon").resizable([false,false]).show(ctx, |ui| {
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
                    ui.heading("Weapon Name: ");
                    let textfield = ui.text_edit_singleline(&mut self.temp.weapon_name);
                });

                ui.add_space(10.0);
                ui.columns(6, |columns| {
                });

                ui.add_space(20.0);
            });
            if self.state == State::ConfirmCreate {
                match show_confirmation_dialog(ctx, "", "") {
                    ConfirmationResult::Pending => {},
                    ConfirmationResult::Confirmed => {
                    },
                    ConfirmationResult::Cancelled => {
                        self.state = State::Create;
                    },
                };
            }
            if self.state == State::ErrorCreate {
                //let build_status = self.build_status.clone();
                //vif show_info_dialog(ctx, "Race Create Error", &build_status.err().unwrap().to_string(), true) {
                //    self.state = State::Create;
                //}
            }
        }
    }
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

pub enum ConfirmationResult {
    Pending,
    Confirmed,
    Cancelled,
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

fn format_damage_dice(dice: &[Die]) -> String {
    use std::collections::HashMap;
    
    if dice.is_empty() {
        return "—".to_string();
    }
    
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for die in dice {
        *counts.entry(die.value()).or_insert(0) += 1;
    }
    
    let mut die_values: Vec<u8> = counts.keys().copied().collect();
    die_values.sort();
    
    die_values.iter()
        .map(|&value| format!("{}d{}", counts[&value], value))
        .collect::<Vec<_>>()
        .join(" + ")
}
