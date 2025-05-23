use eframe::egui;
use crate::project::Project;
use crate::visualization::VisualizationState;

#[allow(dead_code)] // Ignore unused function warning for the original function
// Keep original function but rename to avoid conflicts
pub fn draw_status_original(ui: &mut egui::Ui, parent_rect: egui::Rect, project: &Project, state: &VisualizationState) {
    let status_height = 25.0;
    let status_rect = egui::Rect::from_min_size(
        egui::pos2(parent_rect.left(), parent_rect.bottom() - status_height),
        egui::vec2(parent_rect.width(), status_height)
    );
    
    // Draw status bar background
    ui.painter().rect_filled(
        status_rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(30, 30, 30, 200)
    );
    
    // Draw statistics
    let text = format!(
        "Elements: {}  |  Relationships: {}  |  Zoom: {:.1}x",
        project.elements.len(),
        project.relationships.len(),
        state.zoom
    );
    
    ui.painter().text(
        status_rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(14.0),
        egui::Color32::from_gray(200)
    );
    
    // Draw project path if available
    if let Some(path) = &project.project_path {
        let path_text = format!("Project: {}", path);
        ui.painter().text(
            egui::pos2(status_rect.left() + 10.0, status_rect.center().y),
            egui::Align2::LEFT_CENTER,
            path_text,
            egui::FontId::proportional(12.0),
            egui::Color32::from_gray(180)
        );
    }
    
    // Draw selected element info
    if let Some(element_id) = &state.selected_element {
        if let Some(element) = project.elements.iter().find(|e| &e.id == element_id) {
            let selected_text = format!("Selected: {}", element.name);
            ui.painter().text(
                egui::pos2(status_rect.right() - 10.0, status_rect.center().y),
                egui::Align2::RIGHT_CENTER,
                selected_text,
                egui::FontId::proportional(12.0),
                egui::Color32::from_gray(220)
            );
        }
    }
}

// Add new function matching the call in renderer.rs
pub fn draw_status(ui: &mut egui::Ui, project: &Project, state: &VisualizationState) {
    let status_height = 25.0;
    let parent_rect = ui.available_rect_before_wrap();
    let status_rect = egui::Rect::from_min_size(
        egui::pos2(parent_rect.left(), parent_rect.bottom() - status_height),
        egui::vec2(parent_rect.width(), status_height)
    );
    
    // Draw status bar background
    ui.painter().rect_filled(
        status_rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(30, 30, 30, 200)
    );
    
    // Draw statistics
    let text = format!(
        "Elements: {}  |  Relationships: {}  |  Zoom: {:.1}x",
        project.elements.len(),
        project.relationships.len(),
        state.zoom
    );
    
    ui.painter().text(
        status_rect.center(),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(14.0),
        egui::Color32::from_gray(200)
    );
    
    // Draw selected element info
    if let Some(element_id) = &state.selected_element {
        if let Some(element) = project.elements.iter().find(|e| &e.id == element_id) {
            let selected_text = format!("Selected: {}", element.name);
            ui.painter().text(
                egui::pos2(status_rect.right() - 10.0, status_rect.center().y),
                egui::Align2::RIGHT_CENTER,
                selected_text,
                egui::FontId::proportional(12.0),
                egui::Color32::from_gray(220)
            );
        }
    }
}
