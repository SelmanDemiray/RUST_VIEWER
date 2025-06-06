use crate::project::ProjectModel;
use crate::visualization::VisualizationState;
use crate::editor::Editor;
use crate::dialog::FileDialog;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ViewMode {
    Visualization,
    Editor,
}

pub struct App {
    pub project: Option<ProjectModel>,
    pub visualization: VisualizationState,
    pub editor: Editor,
    pub view_mode: ViewMode,
    pub show_file_dialog: bool,
    pub file_dialog: Option<FileDialog>,
    pub selected_file: Option<String>,
    pub status_message: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: None,
            visualization: VisualizationState::new(),
            editor: Editor::new(),
            view_mode: ViewMode::Visualization,
            show_file_dialog: false,
            file_dialog: None,
            selected_file: None,
            status_message: "Ready".to_string(),
        }
    }
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Handle file dialog
        if self.show_file_dialog {
            // Create a temporary variable to avoid borrowing conflicts
            let dialog_result = if let Some(ref mut dialog) = self.file_dialog {
                dialog.show(ctx)
            } else {
                self.file_dialog = Some(crate::dialog::FileDialog::new());
                None
            };
            
            // Process the result after the borrow is complete
            if let Some(path) = dialog_result {
                if path.is_empty() {
                    // User cancelled
                    self.show_file_dialog = false;
                    self.file_dialog = None;
                } else {
                    // User selected a folder
                    match crate::project::loader::load_project_from_path(&path) {
                        Ok(project) => {
                            self.project = Some(project);
                            self.status_message = format!("Loaded project from: {}", path);
                        },
                        Err(e) => {
                            self.status_message = format!("Error loading project: {}", e);
                        }
                    }
                    self.show_file_dialog = false;
                    self.file_dialog = None;
                }
            }
            
            // Check if dialog should be closed
            if let Some(ref dialog) = self.file_dialog {
                if !dialog.is_open() {
                    self.show_file_dialog = false;
                    self.file_dialog = None;
                }
            }
        }
        
        // Render UI
        crate::ui::top_panel::render(self, ctx);
        
        eframe::egui::SidePanel::left("side_panel")
            .resizable(true)
            .default_width(200.0)
            .show(ctx, |ui| {
                crate::ui::side_panel::render(self, ui);
            });
        
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            crate::ui::central_panel::render(self, ui);
        });
    }
}
