use eframe::egui::{self, CentralPanel, ViewportBuilder};

#[derive(Default)]
struct App{
    
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.heading("hello"));
    }
}

fn main() -> Result<(), eframe::Error> {
    println!("sup");
    let options = eframe::NativeOptions{
        viewport: ViewportBuilder::default().with_title("D&D Character Tracker").with_inner_size([320.0, 600.0]).with_min_inner_size([320.0, 600.0]), 
        ..Default::default()
    };

    eframe::run_native("D&D Character Tracker", options, Box::new(|_|Ok(Box::<App>::default())))
}
