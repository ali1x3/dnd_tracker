use std::{cell::RefCell, rc::Rc};

use eframe::{egui::{Button, Color32, Layout, RichText, Stroke}, epaint::MarginF32};

use crate::{db::models::{self, Die, Weapon as WeaponModel}, Model};

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
    weight: String,
    description: String,
    properties: Vec<String>,
    damage_type: String,
    new_property_input: String,
}

impl TempWeapon {
    fn new() -> Self {
        Self {
            weapon_name: String::new(),
            damage_dice: Vec::new(),
            weight: String::from("1.0"),
            description: String::new(),
            properties: Vec::new(),
            damage_type: String::new(),
            new_property_input: String::new(),
        }
    }

    fn clear(&mut self) {
        *self = Self::new();
    }
}

pub enum State {
    View,
    Create,
    ConfirmCreate,
    ErrorCreate,
    ConfirmDelete,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Weapon {
    pub fn new(model: Rc<RefCell<Model>>, command_sender: Sender<Command>) -> Self {
        Self {
            model,
            command_sender,
            state: State::View,
            temp: TempWeapon::new(),
        }
    }

    fn render_view(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        let selected_weapon = self.model.borrow().selected_weapon.clone();
        
        match &selected_weapon {
            Some(weapon) => {
                // Header with weapon name and buttons
                make_frame(MarginF32{ left: 30.0, right: 0.0, top: 20.0, bottom: 20.0 }).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(text_builder(&weapon.weapon_name, 30.0).strong());
                        ui.add_space(ui.available_width() * 0.8);

                        if ui.add_sized([70.0, 30.0], Button::new(RichText::new("Edit").size(20.0))).clicked() {
                            // TODO: Implement edit
                        }

                        if ui.add_sized([70.0, 30.0], Button::new(RichText::new("Delete").size(20.0))).clicked() {
                            self.state = State::ConfirmDelete;
                        }
                    });
                });

                // Damage display
                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(text_builder("Damage:", 25.0).strong());
                        ui.heading(text_builder(&format_damage_dice(&weapon.damage_dice), 25.0)
                            .color(Color32::from_rgb(255, 200, 100)));
                        ui.add_space(20.0);
                        ui.heading(text_builder(&format!("({})", weapon.damage_type), 22.0));
                    });
                });

                // Weight
                make_frame(MarginF32{ left: 30.0, right: 100.0, top: 10.0, bottom: 10.0 }).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.heading(text_builder("Weight:", 25.0).strong());
                        ui.heading(text_builder(&format!("{} lb", weapon.weight), 25.0));
                    });
                });

                // Description and Properties in columns
                ui.columns(2, |columns| {
                    // Left column - Description
                    make_frame(MarginF32::symmetric(30.0, 10.0)).show(&mut columns[0], |ui| {
                        ui.heading(text_builder("Description", 25.0).strong());
                        ui.add_space(10.0);
                        ui.label(text_builder(&weapon.description, 18.0));
                    });

                    // Right column - Properties
                    make_frame(MarginF32::symmetric(30.0, 10.0)).show(&mut columns[1], |ui| {
                        ui.heading(text_builder("Properties", 25.0).strong());
                        ui.add_space(10.0);
                        
                        if weapon.properties.is_empty() {
                            ui.label(text_builder("  None", 18.0));
                        } else {
                            for property in &weapon.properties {
                                ui.label(text_builder(&format!("  • {}", property), 18.0));
                            }
                        }
                    });
                });
            }
            None => {
                ui.vertical_centered(|ui| {
                    ui.add_space(100.0);
                    ui.heading(text_builder("Select a weapon to view details", 25.0));
                });
            }
        }
    }

    fn render_create_dialog(&mut self, ctx: &egui::Context) {
        egui::Window::new("Create Weapon")
            .resizable([false, false])
            .show(ctx, |ui| {
                ui.set_min_size([900.0, 700.0].into());
                ui.set_max_size([900.0, 700.0].into());
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    // Header buttons
                    ui.horizontal(|ui| {
                        ui.heading("");
                        ui.add_space(ui.available_width() - 170.0);

                        if ui.add_sized([90.0, 30.0], Button::new(RichText::new("Confirm").size(20.0))).clicked() {
                            self.state = State::ConfirmCreate;
                        }

                        if ui.add_sized([70.0, 30.0], Button::new(RichText::new("Exit").size(20.0))).clicked() {
                            self.temp.clear();
                            self.state = State::View;
                        }
                    });

                    ui.separator();
                    ui.add_space(10.0);

                    // Weapon Name
                    ui.horizontal(|ui| {
                        ui.label(text_builder("Weapon Name:", 18.0));
                        ui.text_edit_singleline(&mut self.temp.weapon_name);
                    });

                    ui.add_space(10.0);

                    // Weight and Damage Type
                    ui.horizontal(|ui| {
                        ui.label(text_builder("Weight:", 18.0));
                        ui.add_sized([80.0, 20.0], egui::TextEdit::singleline(&mut self.temp.weight));
                        ui.label("lb");
                        
                        ui.add_space(30.0);
                        
                        ui.label(text_builder("Damage Type:", 18.0));
                        ui.text_edit_singleline(&mut self.temp.damage_type);
                    });

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    // Damage Dice Configurator
                    self.render_damage_dice_configurator(ui);

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    // Properties
                    ui.heading(text_builder("Properties", 20.0).strong());
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.text_edit_singleline(&mut self.temp.new_property_input);
                        if ui.button("Add Property").clicked() && !self.temp.new_property_input.trim().is_empty() {
                            self.temp.properties.push(self.temp.new_property_input.trim().to_string());
                            self.temp.new_property_input.clear();
                        }
                    });

                    let mut property_to_remove: Option<usize> = None;
                    for (idx, property) in self.temp.properties.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(format!("  • {}", property));
                            if ui.button("Remove").clicked() {
                                property_to_remove = Some(idx);
                            }
                        });
                    }
                    if let Some(idx) = property_to_remove {
                        self.temp.properties.remove(idx);
                    }

                    ui.add_space(20.0);
                    ui.separator();
                    ui.add_space(20.0);

                    // Description
                    ui.label(text_builder("Description:", 18.0));
                    ui.text_edit_multiline(&mut self.temp.description);
                });
            });
    }

    fn render_damage_dice_configurator(&mut self, ui: &mut egui::Ui) {
        ui.heading(text_builder("Damage Dice", 20.0).strong());
        ui.add_space(10.0);
        
        // Current damage display
        egui::Frame::group(ui.style())
            .fill(Color32::from_rgb(40, 40, 45))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(text_builder("Current Damage:", 18.0).strong());
                    ui.label(text_builder(&format_damage_dice(&self.temp.damage_dice), 20.0)
                        .color(Color32::LIGHT_GREEN));
                });
            });
        
        ui.add_space(15.0);
        
        // Dice selector buttons
        ui.label(text_builder("Add Dice:", 18.0));
        ui.horizontal_wrapped(|ui| {
            for die in Die::all() {
                let button = Button::new(RichText::new(format!("d{}", die.get_value())).size(16.0));
                if ui.add_sized([50.0, 35.0], button).clicked() {
                    self.temp.damage_dice.push(die);
                }
            }
        });
        
        ui.add_space(15.0);
        
        // Display added dice with remove buttons
        if !self.temp.damage_dice.is_empty() {
            ui.label(text_builder("Added Dice:", 18.0));
            ui.add_space(5.0);
            
            let mut index_to_remove: Option<usize> = None;
            
            egui::ScrollArea::vertical()
                .max_height(150.0)
                .show(ui, |ui| {
                    for (idx, die) in self.temp.damage_dice.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(text_builder(&format!("d{}", die.get_value()), 18.0));
                            ui.add_space(10.0);
                            
                            if ui.add_sized([60.0, 25.0], 
                                Button::new(RichText::new("Remove").size(14.0))
                                    .fill(Color32::from_rgb(150, 50, 50))
                            ).clicked() {
                                index_to_remove = Some(idx);
                            }
                        });
                    }
                });
            
            if let Some(idx) = index_to_remove {
                self.temp.damage_dice.remove(idx);
            }
            
            ui.add_space(10.0);
            
            // Quick clear all button
            ui.horizontal(|ui| {
                if ui.button("Clear All Dice").clicked() {
                    self.temp.damage_dice.clear();
                }
            });
        }
    }

    fn try_create_weapon(&mut self) -> Result<WeaponModel, String> {
        // Validate
        if self.temp.weapon_name.trim().is_empty() {
            return Err("Weapon name is required".to_string());
        }
        
        if self.temp.damage_dice.is_empty() {
            return Err("At least one damage die is required".to_string());
        }
        
        if self.temp.damage_type.trim().is_empty() {
            return Err("Damage type is required".to_string());
        }
        
        let weight = self.temp.weight.parse::<f32>()
            .map_err(|_| "Invalid weight value".to_string())?;
        
        if weight < 0.0 {
            return Err("Weight cannot be negative".to_string());
        }

        // Create weapon
        Ok(WeaponModel {
            id: surrealdb::sql::Thing::from(("weapon", surrealdb::sql::Id::rand())),
            weapon_name: self.temp.weapon_name.trim().to_string(),
            damage_dice: self.temp.damage_dice.clone(),
            weight,
            description: self.temp.description.trim().to_string(),
            properties: self.temp.properties.clone(),
            damage_type: self.temp.damage_type.trim().to_string(),
        })
    }
}

impl Pages for Weapon {
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Render main view
        if self.state == State::View {
            self.render_view(ctx, ui);
        }

        // Render create dialog
        if self.state == State::Create || self.state == State::ConfirmCreate || self.state == State::ErrorCreate {
            self.render_create_dialog(ctx);
        }

        // Handle confirmation dialogs
        if self.state == State::ConfirmCreate {
            match show_confirmation_dialog(ctx, "Confirm Create", "Create this weapon?") {
                ConfirmationResult::Pending => {}
                ConfirmationResult::Confirmed => {
                    match self.try_create_weapon() {
                        Ok(weapon) => {
                            let _ = self.command_sender.send(Command::CreateWeapon(weapon));
                            let _ = self.command_sender.send(Command::LoadWeapons);
                            self.temp.clear();
                            self.state = State::View;
                        }
                        Err(error) => {
                            // Show error (you can implement show_info_dialog here)
                            eprintln!("Error creating weapon: {}", error);
                            self.state = State::ErrorCreate;
                        }
                    }
                }
                ConfirmationResult::Cancelled => {
                    self.state = State::Create;
                }
            }
        }

        if self.state == State::ConfirmDelete {
            match show_confirmation_dialog(ctx, "Delete Weapon?", "Are you sure you want to delete this weapon?") {
                ConfirmationResult::Pending => {}
                ConfirmationResult::Confirmed => {
                    let weapon = self.model.borrow().selected_weapon.clone();
                    if let Some(weapon) = weapon {
                        let _ = self.command_sender.send(Command::DeleteWeapon(weapon.id));
                        self.model.borrow_mut().selected_weapon = None;
                        let _ = self.command_sender.send(Command::LoadWeapons);
                    }
                    self.state = State::View;
                }
                ConfirmationResult::Cancelled => {
                    self.state = State::View;
                }
            }
        }

        if self.state == State::ErrorCreate {
            if show_info_dialog(ctx, "Weapon Create Error", "Failed to create weapon. Please check all fields.", true) {
                self.state = State::Create;
            }
        }
    }
}

// Helper functions
fn show_confirmation_dialog(ctx: &egui::Context, title: &str, message: &str) -> ConfirmationResult {
    let mut result = ConfirmationResult::Pending;
    
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

fn show_info_dialog(ctx: &egui::Context, title: &str, message: &str, is_error: bool) -> bool {
    let mut should_close = false;
    
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
            ui.set_min_width(300.0);
            
            let color = if is_error { Color32::RED } else { Color32::WHITE };
            ui.colored_label(color, message);
            ui.add_space(15.0);
            
            ui.vertical_centered(|ui| {
                if ui.add_sized([80.0, 30.0], Button::new("OK")).clicked() {
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

fn text_builder(text: &str, size: f32) -> RichText {
    RichText::new(text).size(size)
}

fn make_frame(margin: MarginF32) -> egui::Frame {
    egui::Frame::default().inner_margin(margin)
}

fn format_damage_dice(dice: &[Die]) -> String {
    use std::collections::HashMap;
    
    if dice.is_empty() {
        return "No damage".to_string();
    }
    
    let mut counts: HashMap<u8, usize> = HashMap::new();
    for die in dice {
        *counts.entry(die.get_value()).or_insert(0) += 1;
    }
    
    let mut die_values: Vec<u8> = counts.keys().copied().collect();
    die_values.sort();
    
    die_values.iter()
        .map(|&value| format!("{}d{}", counts[&value], value))
        .collect::<Vec<_>>()
        .join(" + ")
}

// Die implementation helpers
impl Die {
    pub fn get_value(&self) -> u8 {
        match self {
            Die::D2 => 2,
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
        }
    }
    
    pub fn all() -> Vec<Die> {
        vec![Die::D2, Die::D4, Die::D6, Die::D8, Die::D10, Die::D12, Die::D20]
    }
}
