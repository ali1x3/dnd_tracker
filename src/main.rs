use anyhow::Error;
use eframe::egui::{self, CentralPanel, ViewportBuilder};
use surrealdb::engine::local::RocksDb;
use surrealdb::Surreal;

mod db;

#[derive(Default)]
struct App{
}

impl eframe::App for App{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.heading("hello"));
    }
}

fn main() -> Result<(), Error> {
    let rt = tokio::runtime::Runtime::new()?;

    let conn_status: Result<(), Error> = rt.block_on(db::init_db());

    match conn_status {
        Ok(_) => println!("db connec"),
        Err(_) => println!("db no connec"),
    }

    let options = eframe::NativeOptions{
        viewport: ViewportBuilder::default()
            .with_title("D&D Character Tracker")
            .with_inner_size([1200.0, 900.0])
            .with_min_inner_size([1200.0, 900.0]), 
        ..Default::default()
    };

    let _ = eframe::run_native("D&D Character Tracker", options, Box::new(|_|Ok(Box::<App>::default())));
    Ok(())
}
