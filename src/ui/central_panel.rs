use eframe::egui;
use crate::app::{App, ViewMode};

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        match app.view_mode {
            ViewMode::Visualization => {
                crate::visualization::render_visualization(ui, &app.project, &mut app.visualization_state);
            },
            ViewMode::Editor => {
                if let Some(file) = &app.selected_file {
                    if let Some(content) = app.project.get_file_content(file) {
                        crate::editor::render_editor(ui, file, content);
                    } else {
                        ui.label("File content not available");
                    }
                } else {
                    ui.centered_and_justified(|ui| {
                        ui.label("Select a file to view its content");
                    });
                }
            }
        }
    });
}
