use eframe::egui;

pub mod renderer;

pub use renderer::render_code_editor;

pub fn render_editor(ui: &mut egui::Ui, file_path: &str, content: &str) {
    ui.vertical(|ui| {
        // Header with file path
        ui.horizontal(|ui| {
            ui.label("File:");
            ui.monospace(file_path);
        });
        
        ui.separator();
        
        // Use the renderer for the actual code display
        renderer::render_code_editor(ui, file_path, content);
    });
}
