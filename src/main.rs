use eframe::egui;
use rust_code_visualizer::App;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1200.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Rust Code Visualizer",
        options,
        Box::new(|_cc| Box::new(App::default())),
    )
}
