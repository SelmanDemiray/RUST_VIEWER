mod parser;
mod project;
mod visualization;
mod editor;
mod simple_dialog;  // New module for our simple file dialog

use eframe::egui;
use project::Project;
use simple_dialog::SimpleFileDialog;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 800.0)),
        ..Default::default()
    };
    
    eframe::run_native(
        "Rust Code Visualizer",
        options,
        Box::new(|_cc| Box::new(RustVisualizer::default())),
    )
}

struct RustVisualizer {
    project: Project,
    selected_file: Option<String>,
    view_mode: ViewMode,
    file_dialog: SimpleFileDialog,  // New field for our custom file dialog
    show_dialog: bool,  // Track dialog visibility
}

enum ViewMode {
    Visualization,
    Editor,
}

impl Default for RustVisualizer {
    fn default() -> Self {
        Self {
            project: Project::default(),
            selected_file: None,
            view_mode: ViewMode::Visualization,
            file_dialog: SimpleFileDialog::default(),
            show_dialog: false,
        }
    }
}

impl eframe::App for RustVisualizer {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open Project").clicked() {
                    // Instead of using rfd, show our custom dialog
                    self.show_dialog = true;
                }
                
                ui.separator();
                
                if ui.selectable_label(matches!(self.view_mode, ViewMode::Visualization), "Visualization").clicked() {
                    self.view_mode = ViewMode::Visualization;
                }
                
                if ui.selectable_label(matches!(self.view_mode, ViewMode::Editor), "Editor").clicked() {
                    self.view_mode = ViewMode::Editor;
                }
            });
        });

        // Display the custom file dialog if needed
        if self.show_dialog {
            egui::Window::new("Select Project Directory")
                .collapsible(false)
                .resizable(true)
                .default_size([400.0, 300.0])
                .show(ctx, |ui| {
                    if let Some(path) = self.file_dialog.show(ui) {
                        self.project.load_project(&path);
                        self.show_dialog = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.show_dialog = false;
                    }
                });
        }

        egui::SidePanel::left("file_panel").show(ctx, |ui| {
            ui.heading("Files");
            
            for file_path in &self.project.files {
                let file_name = file_path.split('/').last().unwrap_or(file_path);
                if ui.selectable_label(
                    Some(file_path) == self.selected_file.as_ref(),
                    file_name
                ).clicked() {
                    self.selected_file = Some(file_path.clone());
                }
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view_mode {
                ViewMode::Visualization => {
                    visualization::render_visualization(ui, &self.project);
                },
                ViewMode::Editor => {
                    if let Some(file_path) = &self.selected_file {
                        if let Some(code) = self.project.get_file_content(file_path) {
                            editor::render_editor(ui, file_path, code);
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
}
