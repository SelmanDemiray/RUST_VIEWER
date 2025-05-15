use eframe::egui;
use crate::project::Project;
use crate::parser::{ElementType, RelationshipType};
use std::collections::HashMap;

pub fn render_visualization(ui: &mut egui::Ui, project: &Project) {
    // Create a visualization canvas
    egui::ScrollArea::both().show(ui, |ui| {
        let available_size = ui.available_size();
        let (response, painter) = ui.allocate_painter(
            available_size,
            egui::Sense::click_and_drag(),
        );

        let rect = response.rect;
        
        // Draw background
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(30, 30, 30));
        
        if project.files.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Open a project to visualize its code",
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
            return;
        }
        
        // Create node positions (in a real app, you'd use a proper graph layout algorithm)
        let mut file_positions = HashMap::new();
        let mut element_positions = HashMap::new();
        
        // Position files in a grid
        let mut x = 100.0;
        let mut y = 100.0;
        let file_spacing = 200.0;
        
        for (i, file) in project.files.iter().enumerate() {
            let pos = egui::pos2(x, y);
            file_positions.insert(file.clone(), pos);
            
            // Move to next position
            x += file_spacing;
            if (i + 1) % 3 == 0 {
                x = 100.0;
                y += file_spacing;
            }
        }
        
        // Position code elements below their files
        for element in &project.elements {
            if let Some(file_pos) = file_positions.get(&element.file_path) {
                let element_x = file_pos.x + (element.name.len() as f32 * 2.0);
                let element_y = file_pos.y + 70.0;
                element_positions.insert(element.id.clone(), egui::pos2(element_x, element_y));
            }
        }
        
        // Draw files
        for (file_path, pos) in &file_positions {
            let file_name = file_path.split('/').last().unwrap_or(file_path);
            let file_rect = egui::Rect::from_center_size(
                *pos,
                egui::vec2(100.0, 50.0),
            );
            
            painter.rect_filled(
                file_rect,
                5.0,
                egui::Color32::from_rgb(60, 60, 100),
            );
            
            painter.text(
                file_rect.center(),
                egui::Align2::CENTER_CENTER,
                file_name,
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
        }
        
        // Draw code elements
        for element in &project.elements {
            if let Some(pos) = element_positions.get(&element.id) {
                let color = match element.element_type {
                    ElementType::Function => egui::Color32::from_rgb(100, 180, 100),
                    ElementType::Module => egui::Color32::from_rgb(180, 100, 100),
                    ElementType::Struct => egui::Color32::from_rgb(100, 100, 180),
                    _ => egui::Color32::from_rgb(150, 150, 150),
                };
                
                let element_rect = egui::Rect::from_center_size(
                    *pos,
                    egui::vec2(80.0, 30.0),
                );
                
                painter.rect_filled(
                    element_rect,
                    3.0,
                    color,
                );
                
                painter.text(
                    element_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &element.name,
                    egui::FontId::default(),
                    egui::Color32::BLACK,
                );
            }
        }
        
        // Draw relationships
        for rel in &project.relationships {
            if let (Some(source_pos), Some(target_pos)) = (
                element_positions.get(&rel.source_id),
                element_positions.get(&rel.target_id),
            ) {
                let color = match rel.relationship_type {
                    RelationshipType::Calls => egui::Color32::from_rgb(200, 200, 50),
                    RelationshipType::Imports => egui::Color32::from_rgb(50, 200, 200),
                    RelationshipType::Implements => egui::Color32::from_rgb(200, 50, 200),
                    RelationshipType::Contains => egui::Color32::from_rgb(150, 150, 150),
                };
                
                // Draw the arrow
                painter.line_segment(
                    [*source_pos, *target_pos],
                    egui::Stroke::new(2.0, color),
                );
                
                // Draw arrowhead using circle_filled which is available in egui 0.21.0
                // Calculate direction vector from source to target
                let dir = (*target_pos - *source_pos).normalized();
                // Position the circle at the end of the line
                let tip = *target_pos - dir * 5.0; // Place circle slightly before the end point
                
                // Use circle_filled which is definitely available in egui 0.21.0
                painter.circle_filled(
                    tip,
                    4.0, // Radius - choose appropriate size
                    color,
                );
            }
        }
    });
}
