use std::sync::mpsc::Sender;

use eframe::egui;

use crate::Command;

pub mod classes;
pub mod races;

pub trait Pages {
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame);
}
