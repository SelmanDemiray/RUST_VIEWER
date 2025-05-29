mod view_mode;
mod state;

pub use view_mode::ViewMode;

use eframe::egui;
use crate::project::Project;
use crate::visualization::VisualizationState;

pub struct App {
    pub project: Project,
    pub visualization_state: VisualizationState,
    pub view_mode: ViewMode,
    pub selected_file: Option<String>,
    pub show_dialog: bool,
    file_dialog: Option<crate::dialog::FileDialog>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            project: Project::default(),
            visualization_state: VisualizationState::default(),
            view_mode: ViewMode::Visualization,
            selected_file: None,
            show_dialog: false,
            file_dialog: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle file dialog
        if self.show_dialog && self.file_dialog.is_none() {
            self.file_dialog = Some(crate::dialog::FileDialog::new());
            self.show_dialog = false;
        }

        if let Some(ref mut dialog) = self.file_dialog {
            if let Some(path) = dialog.show(ctx) {
                if !path.is_empty() {
                    self.load_project(path);
                }
                self.file_dialog = None;
            } else if !dialog.is_open() {
                self.file_dialog = None;
            }
        }

        // Top panel
        crate::ui::top_panel::render(self, ctx);
        
        // Side panel
        crate::ui::side_panel::render(self, ctx);
        
        // Central panel
        crate::ui::central_panel::render(self, ctx);
    }
}

impl App {
    fn load_project(&mut self, path: String) {
        self.project.load_project(&path);
        self.visualization_state = VisualizationState::default();
        self.selected_file = self.project.files.first().cloned();
    }
}
