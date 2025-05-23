use crate::{
    dialog::SimpleFileDialog,
    project::Project,
    visualization::state::VisualizationState,
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
            visualization_state: VisualizationState::default(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Top panel
        crate::ui::top_panel::render(self, ctx);
        
        // File dialog
        if self.show_dialog {
            crate::ui::render_file_dialog(self, ctx);
        }
        
        // Side panel
        crate::ui::side_panel::render(self, ctx);
        
        // Central panel
        crate::ui::central_panel::render(self, ctx);
    }
}
