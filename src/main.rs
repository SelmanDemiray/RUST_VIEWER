use eframe::egui;
use rust_code_visualizer::App;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        min_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rust Code Visualizer",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
