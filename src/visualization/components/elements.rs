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
    
    // Draw file background circles first
    draw_file_backgrounds(painter, project, element_positions, state.zoom);
    
    // Draw elements with improved styling
    for element in &project.elements {
        if let Some(pos) = element_positions.get(&element.id) {
            let is_selected = state.selected_element.as_ref() == Some(&element.id);
            let is_hovered = response.hovered() && 
                response.hover_pos().map_or(false, |mouse_pos| {
                    (mouse_pos - *pos).length() < get_element_radius(&element.element_type, state.zoom) + 5.0
                });
            
            // Filter elements based on search text
            let should_show = state.filter_text.is_empty() || 
                element.name.to_lowercase().contains(&state.filter_text.to_lowercase()) ||
                element.file_path.to_lowercase().contains(&state.filter_text.to_lowercase());
            
            if !should_show {
                continue;
            }
            
            draw_element(painter, element, *pos, is_selected, is_hovered, state);
            
            // Handle clicks for selection
            if response.clicked() && is_hovered {
                state.selected_element = Some(element.id.clone());
            }
        }
    }
}

fn draw_file_backgrounds(
    painter: &egui::Painter,
    project: &Project,
    element_positions: &HashMap<String, egui::Pos2>,
    zoom: f32,
) {
    // Group elements by file
    let mut file_bounds: HashMap<String, (egui::Pos2, egui::Pos2)> = HashMap::new();
    
    for element in &project.elements {
        if let Some(pos) = element_positions.get(&element.id) {
            let entry = file_bounds.entry(element.file_path.clone()).or_insert((*pos, *pos));
            entry.0.x = entry.0.x.min(pos.x);
            entry.0.y = entry.0.y.min(pos.y);
            entry.1.x = entry.1.x.max(pos.x);
            entry.1.y = entry.1.y.max(pos.y);
        }
    }
    
    // Draw file background areas
    for (file_path, (min_pos, max_pos)) in file_bounds {
        let padding = 30.0 * zoom;
        let rect = egui::Rect::from_min_max(
            egui::pos2(min_pos.x - padding, min_pos.y - padding),
            egui::pos2(max_pos.x + padding, max_pos.y + padding)
        );
        
        // Draw subtle background
        painter.rect_filled(
            rect,
            8.0 * zoom,
            egui::Color32::from_rgba_unmultiplied(40, 40, 60, 50)
        );
        
        painter.rect_stroke(
            rect,
            8.0 * zoom,
            egui::Stroke::new(1.0 * zoom, egui::Color32::from_rgba_unmultiplied(80, 80, 120, 100))
        );
        
        // Draw file label
        if zoom > 0.3 {
            let file_name = file_path.split('/').last().unwrap_or(&file_path);
            painter.text(
                egui::pos2(rect.center().x, rect.min.y + 15.0 * zoom),
                egui::Align2::CENTER_CENTER,
                file_name,
                egui::FontId::proportional(12.0 * zoom.min(1.2)),
                egui::Color32::from_gray(180)
            );
        }
    }
}

fn draw_element(
    painter: &egui::Painter,
    element: &crate::project::Element,
    pos: egui::Pos2,
    is_selected: bool,
    is_hovered: bool,
    state: &VisualizationState,
) {
    // Calculate element appearance
    let (base_color, shape) = get_element_style(&element.element_type);
    let radius = get_element_radius(&element.element_type, state.zoom);
    
    let color = if is_selected {
        egui::Color32::WHITE
    } else if is_hovered {
        base_color.linear_multiply(1.4)
    } else {
        base_color
    };
    
    let final_radius = radius * if is_selected { 1.3 } else { 1.0 };
    
    // Draw element based on shape
    match shape {
        ElementShape::Circle => {
            painter.circle_filled(pos, final_radius, color);
        },
        ElementShape::Square => {
            let rect = egui::Rect::from_center_size(pos, egui::vec2(final_radius * 2.0, final_radius * 2.0));
            painter.rect_filled(rect, 3.0, color);
        },
        ElementShape::Diamond => {
            let points = vec![
                pos + egui::vec2(0.0, -final_radius),
                pos + egui::vec2(final_radius, 0.0),
                pos + egui::vec2(0.0, final_radius),
                pos + egui::vec2(-final_radius, 0.0),
            ];
            painter.add(egui::epaint::PathShape::convex_polygon(
                points,
                color,
                egui::Stroke::NONE,
            ));
        },
        ElementShape::Triangle => {
            let height = final_radius * 1.5;
            let points = vec![
                pos + egui::vec2(0.0, -height * 0.6),
                pos + egui::vec2(final_radius * 0.8, height * 0.4),
                pos + egui::vec2(-final_radius * 0.8, height * 0.4),
            ];
            painter.add(egui::epaint::PathShape::convex_polygon(
                points,
                color,
                egui::Stroke::NONE,
            ));
        },
    }
    
    // Draw border for important elements or selection
    if is_selected || is_hovered || matches!(element.element_type, ElementType::Struct | ElementType::Trait | ElementType::Module) {
        let border_color = if is_selected {
            egui::Color32::from_rgb(255, 255, 100)
        } else if is_hovered {
            egui::Color32::WHITE
        } else {
            color.linear_multiply(0.7)
        };
        
        match shape {
            ElementShape::Circle => {
                painter.circle_stroke(pos, final_radius + 2.0, egui::Stroke::new(2.0, border_color));
            },
            ElementShape::Square => {
                let rect = egui::Rect::from_center_size(pos, egui::vec2(final_radius * 2.0 + 4.0, final_radius * 2.0 + 4.0));
                painter.rect_stroke(rect, 3.0, egui::Stroke::new(2.0, border_color));
            },
            _ => {
                painter.circle_stroke(pos, final_radius + 2.0, egui::Stroke::new(2.0, border_color));
            }
        }
    }
    
    // Draw labels with better visibility
    draw_element_label(painter, element, pos, state.should_draw_labels(), is_selected, is_hovered, state.zoom);
}

#[derive(Clone, Copy)]
enum ElementShape {
    Circle,
    Square,
    Diamond,
    Triangle,
}

fn get_element_style(element_type: &ElementType) -> (egui::Color32, ElementShape) {
    match element_type {
        ElementType::Function => (egui::Color32::from_rgb(100, 170, 255), ElementShape::Circle),
        ElementType::Struct => (egui::Color32::from_rgb(255, 150, 100), ElementShape::Square),
        ElementType::Enum => (egui::Color32::from_rgb(150, 255, 100), ElementShape::Diamond),
        ElementType::Trait => (egui::Color32::from_rgb(255, 100, 180), ElementShape::Triangle),
        ElementType::Impl => (egui::Color32::from_rgb(180, 100, 255), ElementShape::Circle),
        ElementType::Module => (egui::Color32::from_rgb(255, 200, 100), ElementShape::Square),
    }
}

fn get_element_radius(element_type: &ElementType, zoom: f32) -> f32 {
    let base_size = match element_type {
        ElementType::Module => 18.0,
        ElementType::Struct | ElementType::Enum | ElementType::Trait => 14.0,
        ElementType::Function | ElementType::Impl => 10.0,
    };
    
    base_size * zoom.clamp(0.5, 2.0)
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
    let should_draw = should_show_labels || is_selected || is_hovered;
    let min_zoom_for_labels = 0.4;
    
    if should_draw && zoom > min_zoom_for_labels {
        let label_offset = egui::vec2(0.0, -get_element_radius(&element.element_type, zoom) - 15.0);
        let label_pos = pos + label_offset;
        
        // Determine text size and style
        let text_size = if is_selected {
            14.0 * zoom.min(1.5)
        } else if is_hovered {
            12.0 * zoom.min(1.3)
        } else {
            10.0 * zoom.min(1.2)
        };
        
        let font_id = egui::FontId::proportional(text_size);
        let text_color = if is_selected {
            egui::Color32::WHITE
        } else if is_hovered {
            egui::Color32::from_gray(240)
        } else {
            egui::Color32::from_gray(200)
        };
        
        // Draw text background for better readability
        let galley = painter.layout_no_wrap(
            element.name.clone(),
            font_id.clone(),
            text_color,
        );
        
        let text_rect = egui::Rect::from_center_size(
            label_pos,
            galley.size() + egui::vec2(6.0, 3.0),
        );
        
        painter.rect_filled(
            text_rect,
            3.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150),
        );
        
        painter.text(
            label_pos,
            egui::Align2::CENTER_CENTER,
            &element.name,
            font_id,
            text_color,
        );
        
        // Show element type for selected items
        if is_selected && zoom > 0.6 {
            let type_text = format!("{:?}", element.element_type);
            let type_pos = pos + egui::vec2(0.0, get_element_radius(&element.element_type, zoom) + 20.0);
            
            painter.text(
                type_pos,
                egui::Align2::CENTER_CENTER,
                type_text,
                egui::FontId::proportional(9.0 * zoom.min(1.1)),
                egui::Color32::from_gray(160),
            );
        }
    }
}
