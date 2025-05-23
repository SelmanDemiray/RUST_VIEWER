use crate::app::{App, ViewMode};
use eframe::egui;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        match app.view_mode {
            ViewMode::Visualization => {
                // Use the re-exported render_visualization function
                crate::visualization::render_visualization(ui, &app.project, &mut app.visualization_state);
            },
            ViewMode::Editor => {
                if let Some(file_path) = &app.selected_file {
                    if let Some(code) = app.project.get_file_content(file_path) {
                        crate::editor::renderer::render(ui, file_path, code);
                    } else {
                        ui.label("Failed to load file content");
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Select a file to view its code");
                    });
                }
            }
        }
    });
}
