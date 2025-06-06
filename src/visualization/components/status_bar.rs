use eframe::egui;
use crate::{
    project::Project,
    visualization::state::VisualizationState,
};

pub fn draw_status(ui: &mut egui::Ui, project: &Project, state: &VisualizationState) {
    ui.allocate_ui_at_rect(
        egui::Rect::from_min_size(
            ui.available_rect_before_wrap().left_bottom() - egui::vec2(0.0, 25.0),
            egui::vec2(ui.available_width(), 25.0),
        ),
        |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Zoom: {:.1}%", state.zoom * 100.0));
                ui.separator();
                ui.label(format!("Layout: {:?}", state.layout_type));
                ui.separator();
                
                if let Some(selected) = &state.selected_element {
                    ui.label(format!("Selected: {}", selected));
                } else {
                    ui.label("No selection");
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("Elements: {}", project.elements.len()));
                });
            });
        },
    );
}
