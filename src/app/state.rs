use crate::{
    simple_dialog::SimpleFileDialog,
    project::Project,
    visualization::VisualizationState,
};

use eframe::egui;
use super::view_mode::ViewMode;

pub struct App {
    pub project: Project,
    pub selected_file: Option<String>,
    pub view_mode: ViewMode,
    pub file_dialog: SimpleFileDialog,
    pub show_dialog: bool,
    pub visualization_state: VisualizationState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: Project::default(),
            selected_file: None,
            view_mode: ViewMode::Visualization,
            file_dialog: SimpleFileDialog::default(),
            show_dialog: false,
            visualization_state: VisualizationState::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top panel
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Project").clicked() {
                        self.show_dialog = true;
                        ui.close_menu();
                    }
                });
                
                ui.separator();
                
                if ui.selectable_label(self.view_mode == ViewMode::Visualization, "Visualization").clicked() {
                    self.view_mode = ViewMode::Visualization;
                }
                
                if ui.selectable_label(self.view_mode == ViewMode::Editor, "Editor").clicked() {
                    self.view_mode = ViewMode::Editor;
                }
            });
        });
        
        // File dialog
        if self.show_dialog {
            egui::Window::new("Select Project Directory")
                .collapsible(false)
                .resizable(true)
                .default_width(500.0)
                .default_height(400.0)
                .show(ctx, |ui| {
                    if let Some(path) = self.file_dialog.show(ui) {
                        if path.is_empty() {
                            // User cancelled
                            self.show_dialog = false;
                        } else {
                            // User selected a folder
                            self.show_dialog = false;
                            self.load_project(path);
                        }
                    }
                });
        }
        
        // Side panel
        egui::SidePanel::left("file_panel").show(ctx, |ui| {
            ui.heading("Project Files");
            ui.separator();
            
            if self.project.files.is_empty() {
                ui.label("No project loaded.");
            } else {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for file in &self.project.files {
                        let is_selected = self.selected_file.as_ref() == Some(file);
                        if ui.selectable_label(is_selected, file).clicked() {
                            self.selected_file = Some(file.clone());
                        }
                    }
                });
            }
        });
        
        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.view_mode {
                ViewMode::Visualization => {
                    crate::visualization::render_visualization(ui, &self.project, &mut self.visualization_state);
                },
                ViewMode::Editor => {
                    if let Some(file) = &self.selected_file {
                        if let Some(content) = self.project.get_file_content(file) {
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
}

impl App {
    fn load_project(&mut self, path: String) {
        self.project.load_project(&path);
        self.visualization_state = VisualizationState::new();
        self.selected_file = self.project.files.first().cloned();
    }
}
