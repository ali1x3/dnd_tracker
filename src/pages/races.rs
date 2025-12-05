use std::{cell::RefCell, rc::Rc};

use super::*;
use crate::{Model, db::models};

pub struct Race {
    model: Rc<RefCell<Model>>,
    command_sender: Sender<Command>,
}

impl Race {
    pub fn new(model: Rc<RefCell<Model>>, command_sender: Sender<Command>) -> Self {
        Race { model, command_sender }
    }
}

impl Pages for Race {
    fn update(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        match &self.model.borrow().selected_class {
            Some(race) => {
                ui.heading(format!("{:#?}", race));
            }
            None => {
                ui.heading("click on a race!");
            }
        }
    }
}
