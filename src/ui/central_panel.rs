use crate::app::state::{App, ViewMode};
use eframe::egui;

pub fn render(app: &mut App, ui: &mut egui::Ui) {
    match app.view_mode {
        ViewMode::Visualization => {
            if let Some(project) = &app.project {
                ui.heading("Project Visualization");
                ui.separator();
                
                // Show project statistics
                ui.horizontal(|ui| {
                    ui.label(format!("Files: {}", project.files.len()));
                    ui.separator();
                    ui.label(format!("Elements: {}", project.elements.len()));
                    ui.separator();
                    ui.label(format!("Relationships: {}", project.relationships.len()));
                });
                
                ui.separator();
                
                // Update visualization state from project
                app.visualization.update_from_project(project);
                
                // Render the visualization
                app.visualization.render(ui);
                
                // Show helpful message if no elements found
                if project.elements.is_empty() {
                    ui.separator();
                    ui.vertical_centered(|ui| {
                        ui.label(egui::RichText::new("No Rust code elements found in the project.")
                            .color(egui::Color32::YELLOW));
                        ui.label("Make sure the project contains .rs files with functions, structs, etc.");
                    });
                }
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading("No Project Loaded");
                    ui.add_space(20.0);
                    ui.label("Please open a project folder to begin visualization");
                    ui.add_space(10.0);
                    if ui.button("Open Project").clicked() {
                        app.show_file_dialog = true;
                    }
                });
            }
        }
        ViewMode::Editor => {
            if let Some(ref project) = app.project {
                app.editor.render(ui, project);
            } else {
                ui.vertical_centered(|ui| {
                    ui.heading("Code Editor");
                    ui.add_space(20.0);
                    ui.label("Please open a project folder to start editing");
                    ui.add_space(10.0);
                    if ui.button("Open Project").clicked() {
                        app.show_file_dialog = true;
                    }
                });
            }
        }
    }
    
    // Status bar
    ui.separator();
    ui.horizontal(|ui| {
        ui.label("Status:");
        ui.label(&app.status_message);
    });
}
