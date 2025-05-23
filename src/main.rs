mod app;
mod dialog;
mod editor;
mod parser;
mod project;
mod simple_dialog;
mod ui;
mod visualization;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Rust Code Visualizer",
        options,
        Box::new(|_cc| Box::new(app::App::default())),
    )
}
