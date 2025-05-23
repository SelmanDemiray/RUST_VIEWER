use eframe::egui;
use crate::project::Project;
use crate::parser::RelationshipType;
use crate::visualization::VisualizationState;
use std::collections::HashMap;

pub fn draw_relationships(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &HashMap<String, egui::Pos2>,
    selected_element: Option<&String>,
    show_all_relationships: bool
) {
    // Implementation for the draw function
    // This should contain the relationship drawing logic from elsewhere
    for relationship in &project.relationships {
        // Find source and target positions
        let source_pos = element_positions.get(&relationship.source_id);
        let target_pos = element_positions.get(&relationship.target_id);
        
        if let (Some(source_pos), Some(target_pos)) = (source_pos, target_pos) {
            // Draw relationship only if it's connected to the selected element or show_all_relationships is true
            let is_selected = selected_element.map_or(false, |id| 
                &relationship.source_id == id || &relationship.target_id == id);
                
            if show_all_relationships || is_selected {
                // Get appropriate stroke style based on relationship type
                let stroke = match relationship.relationship_type {
                    RelationshipType::Calls => egui::Stroke::new(
                        if is_selected { 2.0 } else { 1.0 },
                        egui::Color32::from_rgba_unmultiplied(200, 200, 255, 180)
                    ),
                    RelationshipType::Imports => egui::Stroke::new(
                        if is_selected { 2.0 } else { 1.0 },
                        egui::Color32::from_rgba_unmultiplied(200, 255, 200, 180)
                    ),
                    RelationshipType::Implements => egui::Stroke::new(
                        if is_selected { 3.0 } else { 2.0 },
                        egui::Color32::from_rgba_unmultiplied(255, 200, 200, 180)
                    ),
                    RelationshipType::Contains => egui::Stroke::new(
                        if is_selected { 4.0 } else { 3.0 },
                        egui::Color32::from_rgba_unmultiplied(255, 255, 200, 180)
                    ),
                };
                
                // Draw the line with the appropriate stroke
                painter.line_segment([*source_pos, *target_pos], stroke);
                
                // Draw an arrow at the target end for directional relationships
                if relationship.relationship_type != RelationshipType::Contains {
                    let dir = (*target_pos - *source_pos).normalized();
                    let arrow_size = if is_selected { 12.0 } else { 8.0 };
                    let arrow_pos = *target_pos - dir * arrow_size * 0.5;
                    let arrow_left = arrow_pos + egui::vec2(dir.y, -dir.x) * arrow_size * 0.5;
                    let arrow_right = arrow_pos + egui::vec2(-dir.y, dir.x) * arrow_size * 0.5;
                    
                    // Create an arrow triangle
                    let arrow_color = stroke.color;
                    let arrow_triangle = egui::epaint::PathShape::convex_polygon(
                        vec![*target_pos, arrow_left, arrow_right],
                        arrow_color.linear_multiply(0.8), // Apply transparency for fill color
                        egui::Stroke::new(1.0, arrow_color),
                    );
                    
                    painter.add(arrow_triangle);
                }
            }
        }
    }
}

#[allow(dead_code)]
// Keep the original function but rename it to avoid conflicts
pub fn draw_relationships_with_ui(_ui: &mut egui::Ui, painter: &egui::Painter, project: &Project, state: &mut VisualizationState) {
    let zoom = state.zoom;
    let pan_offset = state.pan_offset;
    
    // Draw relationships between elements
    for relationship in &project.relationships {
        // Find source and target elements
        let source = project.elements.iter().find(|e| e.id == relationship.source_id);
        let target = project.elements.iter().find(|e| e.id == relationship.target_id);
        
        if let (Some(_source), Some(_target)) = (source, target) {
            // Simplified positioning - in a real app you'd calculate actual positions
            let source_pos = egui::pos2(100.0 * zoom + pan_offset.x, 100.0 * zoom + pan_offset.y);
            let target_pos = egui::pos2(300.0 * zoom + pan_offset.x, 200.0 * zoom + pan_offset.y);
            
            // Draw a line for the relationship
            let stroke = match relationship.relationship_type {
                RelationshipType::Calls => egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(200, 200, 255, 180)),
                RelationshipType::Imports => egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(200, 255, 200, 180)),
                RelationshipType::Implements => egui::Stroke::new(2.0, egui::Color32::from_rgba_unmultiplied(255, 200, 200, 180)),
                RelationshipType::Contains => egui::Stroke::new(3.0, egui::Color32::from_rgba_unmultiplied(255, 255, 200, 180)),
            };
            
            painter.line_segment([source_pos, target_pos], stroke);
            
            // Draw an arrow at the target end
            let dir = (target_pos - source_pos).normalized();
            let arrow_size = 10.0 * zoom;
            let arrow_pos = target_pos - dir * arrow_size * 0.5;
            let arrow_left = arrow_pos + egui::vec2(dir.y, -dir.x) * arrow_size * 0.5;
            let arrow_right = arrow_pos + egui::vec2(-dir.y, dir.x) * arrow_size * 0.5;
            
            // Create an arrow triangle
            let arrow_triangle = egui::epaint::PathShape::convex_polygon(
                vec![target_pos, arrow_left, arrow_right],
                egui::Color32::from_rgba_unmultiplied(180, 180, 180, 140), // Slightly transparent fill
                egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(180, 180, 180, 180)),
            );
            
            painter.add(arrow_triangle);
        }
    }
}

#[allow(dead_code)]
pub fn draw(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &std::collections::HashMap<String, egui::Pos2>,
    selected_element: Option<&String>,
    show_all_relationships: bool
) {
    // Implementation for the draw function
    // This should contain the relationship drawing logic from elsewhere
    for relationship in &project.relationships {
        // Find source and target positions
        let source_pos = element_positions.get(&relationship.source_id);
        let target_pos = element_positions.get(&relationship.target_id);
        
        if let (Some(source_pos), Some(target_pos)) = (source_pos, target_pos) {
            // Draw relationship only if it's connected to the selected element or show_all_relationships is true
            let is_selected = selected_element.map_or(false, |id| 
                &relationship.source_id == id || &relationship.target_id == id);
                
            if show_all_relationships || is_selected {
                // Draw the line
                painter.line_segment(
                    [*source_pos, *target_pos],
                    egui::Stroke::new(
                        if is_selected { 2.0 } else { 1.0 },
                        egui::Color32::from_rgba_unmultiplied(200, 200, 255, 180)
                    )
                );
            }
        }
    }
}
