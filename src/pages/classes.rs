use super::*;

pub struct Class {}

impl Pages for Class {
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        ui.heading("this is class");
    }
}
