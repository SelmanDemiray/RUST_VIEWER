use eframe::egui;
use crate::project::Project;
use crate::visualization::{render_visualization, VisualizationState};

pub struct App {
    project: Project,
    visualization_state: VisualizationState,
    current_view: ViewType,
    selected_file: Option<String>,
    file_dialog: Option<crate::dialog::FileDialog>, // Changed to Option
}

#[derive(PartialEq)]
pub enum ViewType {
    Visualization,
    Editor,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: Project::default(),
            visualization_state: VisualizationState::default(),
            current_view: ViewType::Visualization,
            selected_file: None,
            file_dialog: None, // Initialize as None
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top panel with application controls
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Open Project").clicked() {
                        self.file_dialog = Some(crate::dialog::FileDialog::new());
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        frame.close();
                    }
                });
                
                ui.separator();
                
                if ui.selectable_label(self.current_view == ViewType::Visualization, "Visualization").clicked() {
                    self.current_view = ViewType::Visualization;
                }
                
                if ui.selectable_label(self.current_view == ViewType::Editor, "Editor").clicked() {
                    self.current_view = ViewType::Editor;
                }
            });
        });
        
        // Left panel with file list
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
        
        // Main central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                ViewType::Visualization => {
                    // Render the visualization using our components
                    render_visualization(ui, &self.project, &mut self.visualization_state);
                },
                ViewType::Editor => {
                    // Show the editor if a file is selected
                    if let Some(file) = &self.selected_file {
                        if let Some(content) = self.project.get_file_content(file) {
                            // Use the editor component
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
        
        // Handle file dialog
        if let Some(ref mut dialog) = self.file_dialog {
            if let Some(path) = dialog.show(ctx) {
                self.load_project(path);
                self.file_dialog = None; // Close dialog after selection
            } else if !dialog.is_open() {
                self.file_dialog = None; // Close dialog if user cancelled
            }
        }
    }
}

impl App {
    // Remove the old handle_file_dialog method as it's no longer needed
    
    fn load_project(&mut self, path: String) {
        self.project.load_project(&path);
        self.visualization_state = VisualizationState::default(); // Reset visualization state
        self.selected_file = self.project.files.first().cloned();
    }
}
