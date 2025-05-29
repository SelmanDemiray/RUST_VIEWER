use eframe::egui;
use std::collections::HashMap;

use crate::{
    parser::ElementType,
    project::Project,
    visualization::state::VisualizationState,
};

pub fn draw(
    ui: &mut egui::Ui,
    project: &Project,
    state: &mut VisualizationState,
    element_positions: &HashMap<String, egui::Pos2>,
    response: &egui::Response,
) {
    let painter = ui.painter();
    
    // Draw elements
    for element in &project.elements {
        if let Some(pos) = element_positions.get(&element.id) {
            let is_selected = state.selected_element.as_ref() == Some(&element.id);
            let is_hovered = response.hovered() && 
                response.hover_pos().map_or(false, |mouse_pos| {
                    (mouse_pos - *pos).length() < 20.0
                });
            
            // Filter elements based on search text
            let should_show = state.filter_text.is_empty() || 
                element.name.to_lowercase().contains(&state.filter_text.to_lowercase()) ||
                element.file_path.to_lowercase().contains(&state.filter_text.to_lowercase());
            
            if !should_show {
                continue;
            }
            
            // Calculate element color based on type
            let base_color = match element.element_type {
                ElementType::Function => egui::Color32::from_rgb(100, 150, 255),
                ElementType::Struct => egui::Color32::from_rgb(255, 150, 100),
                ElementType::Enum => egui::Color32::from_rgb(150, 255, 100),
                ElementType::Trait => egui::Color32::from_rgb(255, 100, 150),
                ElementType::Impl => egui::Color32::from_rgb(150, 100, 255),
                ElementType::Module => egui::Color32::from_rgb(200, 200, 100),
            };
            
            let color = if is_selected {
                egui::Color32::WHITE
            } else if is_hovered {
                base_color.linear_multiply(1.3)
            } else {
                base_color
            };
            
            // Calculate size based on zoom and selection
            let base_size = match element.element_type {
                ElementType::Module => 15.0,
                ElementType::Struct | ElementType::Enum | ElementType::Trait => 12.0,
                ElementType::Function | ElementType::Impl => 8.0,
            };
            
            let size = base_size * state.zoom * if is_selected { 1.5 } else { 1.0 };
            
            // Draw element as a circle
            painter.circle_filled(*pos, size, color);
            
            // Draw border for selected/hovered elements
            if is_selected || is_hovered {
                painter.circle_stroke(
                    *pos,
                    size + 2.0,
                    egui::Stroke::new(2.0, egui::Color32::WHITE)
                );
            }
            
            // Draw labels if enabled and zoom is sufficient
            draw_element_label(painter, element, *pos, state.should_draw_labels(), is_selected, is_hovered, state.zoom);
            
            // Handle clicks for selection
            if response.clicked() && is_hovered {
                state.selected_element = Some(element.id.clone());
            }
        }
    }
}

fn draw_element_label(
    painter: &egui::Painter,
    element: &crate::project::Element,
    pos: egui::Pos2,
    should_show_labels: bool,
    is_selected: bool,
    is_hovered: bool,
    zoom: f32,
) {
    if should_show_labels || is_selected || is_hovered {
        if zoom > 0.5 {
            let label_offset = egui::vec2(0.0, -25.0 * zoom);
            let label_pos = pos + label_offset;
            
            // Draw background for better readability
            let text_size = 12.0 * zoom.min(1.5);
            let font_id = egui::FontId::proportional(text_size);
            
            // Calculate text size for background
            let galley = painter.layout_no_wrap(
                element.name.clone(),
                font_id.clone(),
                egui::Color32::WHITE,
            );
            
            let text_rect = egui::Rect::from_center_size(
                label_pos,
                galley.size() + egui::vec2(4.0, 2.0),
            );
            
            // Draw background
            painter.rect_filled(
                text_rect,
                2.0,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 180),
            );
            
            // Draw text
            painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                &element.name,
                font_id,
                egui::Color32::WHITE,
            );
            
            // File path (if selected)
            if is_selected {
                let file_label_pos = pos + egui::vec2(0.0, 25.0 * zoom);
                let file_font_id = egui::FontId::proportional(10.0 * zoom.min(1.2));
                
                painter.text(
                    file_label_pos,
                    egui::Align2::CENTER_CENTER,
                    &element.file_path,
                    file_font_id,
                    egui::Color32::from_gray(180),
                );
            }
        }
    }
}
