use eframe::egui;
use std::collections::HashMap;

use crate::{
    parser::ElementType,
    project::Project,
    visualization::state::VisualizationState,
};

pub fn draw(
    ui: &mut egui::Ui,
    _project: &Project,
    state: &mut VisualizationState,
    element_positions: &HashMap<String, egui::Pos2>,
    response: &egui::Response,
) {
    // Get the label visibility setting once to ensure the field is read
    let should_show_labels = state.show_labels;
    
    // Draw files
    for (file_path, pos) in element_positions {
        let file_name = file_path.split('/').last().unwrap_or(file_path);
        let file_size = egui::vec2(120.0, 40.0) * state.zoom;
        let file_rect = egui::Rect::from_center_size(*pos, file_size);
        
        // Check if element was clicked
        if response.clicked() {
            if let Some(hover_pos) = response.hover_pos() {
                if file_rect.contains(hover_pos) {
                    // Select or deselect this element
                    if state.selected_element.as_ref().map_or(false, |id| id == file_path) {
                        state.selected_element = None;
                    } else {
                        state.selected_element = Some(file_path.clone());
                    }
                }
            }
        }
        
        // Get the painter for drawing the elements
        let painter = ui.painter();
        
        // Draw shadow for 3D effect
        painter.rect_filled(
            file_rect.translate(egui::vec2(3.0, 3.0)),
            5.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        );
        
        // Check if this is the selected element
        let is_selected = state.selected_element.as_ref().map_or(false, |id| file_path.contains(id));
        
        // Draw file background with different color if selected
        painter.rect_filled(
            file_rect,
            5.0 * state.zoom,
            if is_selected {
                egui::Color32::from_rgb(100, 100, 180)
            } else {
                egui::Color32::from_rgb(70, 70, 120)
            },
        );
        
        // Draw file name if labels are enabled or this is the selected element
        if should_show_labels || is_selected {
            painter.text(
                file_rect.center(),
                egui::Align2::CENTER_CENTER,
                file_name,
                egui::FontId::proportional(14.0 * state.zoom),
                egui::Color32::WHITE,
            );
        }
    }
}

#[allow(dead_code)]
pub fn draw_elements(_ui: &mut egui::Ui, painter: &egui::Painter, project: &Project, state: &mut VisualizationState) {
    let zoom = state.zoom;
    let pan_offset = state.pan_offset;
    
    // Draw code elements
    for (i, element) in project.elements.iter().enumerate() {
        // Simplified positioning - in a real app you'd use a proper layout algorithm
        let x = 100.0 + (i as f32 % 5.0) * 200.0;
        let y = 100.0 + (i as f32 / 5.0) * 150.0;
        
        let element_pos = egui::pos2(x * zoom + pan_offset.x, y * zoom + pan_offset.y);
        let element_size = egui::vec2(180.0 * zoom, 80.0 * zoom);
        
        let element_rect = egui::Rect::from_min_size(element_pos, element_size);
        
        // Determine color based on element type
        let fill_color = match element.element_type {
            ElementType::Function => egui::Color32::from_rgba_unmultiplied(60, 60, 120, 200),
            ElementType::Module => egui::Color32::from_rgba_unmultiplied(60, 120, 60, 200),
            ElementType::Struct => egui::Color32::from_rgba_unmultiplied(120, 60, 60, 200),
            ElementType::Enum => egui::Color32::from_rgba_unmultiplied(120, 60, 120, 200),
            ElementType::Trait => egui::Color32::from_rgba_unmultiplied(60, 120, 120, 200),
            ElementType::Impl => egui::Color32::from_rgba_unmultiplied(120, 120, 60, 200),
        };
        
        // Check if this element is selected
        let is_selected = state.selected_element.as_ref().map_or(false, |id| id == &element.id);
        let stroke = if is_selected {
            egui::Stroke::new(2.0, egui::Color32::WHITE)
        } else {
            egui::Stroke::new(1.0, egui::Color32::from_gray(180))
        };
        
        // Draw the element rectangle
        painter.rect(element_rect, 5.0, fill_color, stroke);
        
        // Draw the element name
        let font_size = 16.0 * zoom;
        let text_pos = element_rect.min + egui::vec2(10.0 * zoom, 10.0 * zoom);
        painter.text(
            text_pos, 
            egui::Align2::LEFT_TOP, 
            &element.name, 
            egui::FontId::proportional(font_size),
            egui::Color32::WHITE,
        );
        
        // Draw the element type
        let type_text = format!("{:?}", element.element_type);
        let type_pos = element_rect.min + egui::vec2(10.0 * zoom, 30.0 * zoom);
        painter.text(
            type_pos, 
            egui::Align2::LEFT_TOP, 
            &type_text, 
            egui::FontId::proportional(font_size * 0.8),
            egui::Color32::from_gray(200),
        );
        
        // Draw the file path (shortened)
        let path_parts: Vec<_> = element.file_path.split('/').collect();
        let short_path = if path_parts.len() > 2 {
            format!(".../{}", path_parts.last().unwrap_or(&""))
        } else {
            element.file_path.clone()
        };
        
        let path_pos = element_rect.min + egui::vec2(10.0 * zoom, 50.0 * zoom);
        painter.text(
            path_pos, 
            egui::Align2::LEFT_TOP, 
            &short_path, 
            egui::FontId::proportional(font_size * 0.7),
            egui::Color32::from_gray(180),
        );
    }
}
