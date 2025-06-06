use eframe::egui;
use std::collections::HashMap;
use crate::{
    project::Project,
    visualization::state::VisualizationState,
    parser::ElementType,
};

pub fn draw(
    ui: &mut egui::Ui,
    project: &Project,
    state: &mut VisualizationState,
    element_positions: &HashMap<String, egui::Pos2>,
    response: &egui::Response,
) {
    let painter = ui.painter();
    
    for element in &project.elements {
        if let Some(pos) = element_positions.get(&element.id) {
            let color = get_element_color(&element.element_type);
            let size = get_element_size(&element.element_type) * state.zoom;
            
            // Draw element
            let rect = egui::Rect::from_center_size(*pos, egui::vec2(size, size));
            painter.rect_filled(rect, 5.0, color);
            painter.rect_stroke(rect, 5.0, egui::Stroke::new(1.0, egui::Color32::WHITE));
            
            // Draw label if enabled
            if state.should_draw_labels() && state.zoom > 0.5 {
                painter.text(
                    *pos + egui::vec2(0.0, size / 2.0 + 15.0),
                    egui::Align2::CENTER_TOP,
                    &element.name,
                    egui::FontId::proportional(12.0 * state.zoom),
                    egui::Color32::WHITE,
                );
            }
            
            // Handle selection
            if response.clicked() {
                if let Some(hover_pos) = response.hover_pos() {
                    if rect.contains(hover_pos) {
                        state.selected_element = Some(element.id.clone());
                    }
                }
            }
            
            // Highlight selected element
            if state.selected_element.as_ref() == Some(&element.id) {
                painter.rect_stroke(
                    rect.expand(3.0),
                    5.0,
                    egui::Stroke::new(2.0, egui::Color32::YELLOW),
                );
            }
        }
    }
}

fn get_element_color(element_type: &ElementType) -> egui::Color32 {
    match element_type {
        ElementType::Function => egui::Color32::from_rgb(100, 200, 100),
        ElementType::Struct => egui::Color32::from_rgb(100, 100, 200),
        ElementType::Enum => egui::Color32::from_rgb(200, 100, 100),
        ElementType::Trait => egui::Color32::from_rgb(200, 200, 100),
        ElementType::Impl => egui::Color32::from_rgb(150, 100, 200),
        ElementType::Module => egui::Color32::from_rgb(100, 200, 200),
    }
}

fn get_element_size(element_type: &ElementType) -> f32 {
    match element_type {
        ElementType::Module => 40.0,
        ElementType::Struct | ElementType::Enum | ElementType::Trait => 30.0,
        ElementType::Impl => 25.0,
        ElementType::Function => 20.0,
    }
}
