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
    // First, draw file-to-file relationships
    draw_file_relationships(project, painter, element_positions, show_all_relationships);
    
    // Then draw element-to-element relationships
    draw_element_relationships(project, painter, element_positions, selected_element, show_all_relationships);
}

fn draw_file_relationships(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &HashMap<String, egui::Pos2>,
    show_all_relationships: bool,
) {
    // Group elements by file to get file positions
    let mut file_centers: HashMap<String, egui::Pos2> = HashMap::new();
    
    for element in &project.elements {
        if let Some(element_pos) = element_positions.get(&element.id) {
            let entry = file_centers.entry(element.file_path.clone()).or_insert(*element_pos);
            // Average position of elements in the file
            *entry = egui::pos2(
                (entry.x + element_pos.x) / 2.0,
                (entry.y + element_pos.y) / 2.0,
            );
        }
    }
    
    // Find relationships between files
    let mut file_relationships: HashMap<(String, String), Vec<&RelationshipType>> = HashMap::new();
    
    for relationship in &project.relationships {
        // Find which files contain the source and target elements
        let source_file = project.elements.iter()
            .find(|e| e.id == relationship.source_id)
            .map(|e| &e.file_path);
        let target_file = project.elements.iter()
            .find(|e| e.id == relationship.target_id)
            .map(|e| &e.file_path);
        
        if let (Some(source_file), Some(target_file)) = (source_file, target_file) {
            if source_file != target_file {
                let key = if source_file < target_file {
                    (source_file.clone(), target_file.clone())
                } else {
                    (target_file.clone(), source_file.clone())
                };
                
                file_relationships.entry(key).or_default().push(&relationship.relationship_type);
            }
        }
    }
    
    // Draw file-to-file relationships
    if show_all_relationships {
        for ((file1, file2), rel_types) in file_relationships {
            if let (Some(pos1), Some(pos2)) = (file_centers.get(&file1), file_centers.get(&file2)) {
                let relationship_count = rel_types.len();
                let color = get_file_relationship_color(&rel_types);
                let thickness = 2.0 + (relationship_count as f32 * 0.5).min(4.0);
                
                draw_smooth_connection(
                    painter, 
                    *pos1, 
                    *pos2, 
                    egui::Stroke::new(thickness, color),
                    ArrowStyle::Simple
                );
                
                // Draw relationship count label
                let midpoint = egui::pos2((pos1.x + pos2.x) / 2.0, (pos1.y + pos2.y) / 2.0);
                painter.circle_filled(midpoint, 8.0, egui::Color32::from_rgba_unmultiplied(40, 40, 40, 200));
                painter.text(
                    midpoint,
                    egui::Align2::CENTER_CENTER,
                    relationship_count.to_string(),
                    egui::FontId::proportional(10.0),
                    egui::Color32::WHITE,
                );
            }
        }
    }
}

fn get_file_relationship_color(rel_types: &[&RelationshipType]) -> egui::Color32 {
    // Determine color based on relationship types
    if rel_types.iter().any(|&rt| *rt == RelationshipType::Implements) {
        egui::Color32::from_rgba_unmultiplied(255, 150, 100, 180)
    } else if rel_types.iter().any(|&rt| *rt == RelationshipType::Imports) {
        egui::Color32::from_rgba_unmultiplied(150, 255, 150, 180)
    } else if rel_types.iter().any(|&rt| *rt == RelationshipType::Calls) {
        egui::Color32::from_rgba_unmultiplied(100, 150, 255, 180)
    } else {
        egui::Color32::from_rgba_unmultiplied(200, 200, 200, 180)
    }
}

fn draw_element_relationships(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &HashMap<String, egui::Pos2>,
    selected_element: Option<&String>,
    show_all_relationships: bool
) {
    for relationship in &project.relationships {
        let source_pos = element_positions.get(&relationship.source_id);
        let target_pos = element_positions.get(&relationship.target_id);
        
        if let (Some(source_pos), Some(target_pos)) = (source_pos, target_pos) {
            let is_selected = selected_element.map_or(false, |id| 
                &relationship.source_id == id || &relationship.target_id == id);
            
            // Only draw element relationships if selected or if specifically requested
            if !show_all_relationships && !is_selected {
                continue;
            }
            
            let (stroke, arrow_style) = get_relationship_style(&relationship.relationship_type, is_selected);
            
            // Draw thinner lines for element relationships to reduce visual noise
            let element_stroke = egui::Stroke::new(stroke.width * 0.7, stroke.color);
            draw_smooth_connection(painter, *source_pos, *target_pos, element_stroke, arrow_style);
        }
    }
}

fn get_relationship_style(rel_type: &RelationshipType, is_selected: bool) -> (egui::Stroke, ArrowStyle) {
    let base_thickness = if is_selected { 2.5 } else { 1.5 };
    let alpha = if is_selected { 220 } else { 120 };
    
    match rel_type {
        RelationshipType::Calls => (
            egui::Stroke::new(base_thickness, egui::Color32::from_rgba_unmultiplied(100, 150, 255, alpha)),
            ArrowStyle::Simple
        ),
        RelationshipType::Imports => (
            egui::Stroke::new(base_thickness, egui::Color32::from_rgba_unmultiplied(150, 255, 150, alpha)),
            ArrowStyle::Dashed
        ),
        RelationshipType::Implements => (
            egui::Stroke::new(base_thickness * 1.2, egui::Color32::from_rgba_unmultiplied(255, 150, 100, alpha)),
            ArrowStyle::Double
        ),
        RelationshipType::Contains => (
            egui::Stroke::new(base_thickness * 0.8, egui::Color32::from_rgba_unmultiplied(255, 200, 100, alpha)),
            ArrowStyle::None
        ),
    }
}

#[derive(Clone, Copy)]
enum ArrowStyle {
    None,
    Simple,
    Double,
    Dashed,
}

fn draw_smooth_connection(
    painter: &egui::Painter,
    start: egui::Pos2,
    end: egui::Pos2,
    stroke: egui::Stroke,
    arrow_style: ArrowStyle,
) {
    let diff = end - start;
    let distance = diff.length();
    
    // Don't draw very short connections to reduce visual noise
    if distance < 30.0 {
        return;
    }
    
    // Create a smooth curve instead of straight line
    let control_offset = distance * 0.3;
    let perpendicular = egui::vec2(-diff.y, diff.x).normalized() * control_offset * 0.5;
    
    let control1 = start + diff * 0.25 + perpendicular;
    let control2 = start + diff * 0.75 - perpendicular;
    
    // Draw cubic bezier curve
    draw_bezier_curve(painter, start, control1, control2, end, stroke);
    
    // Draw arrow at the end
    if !matches!(arrow_style, ArrowStyle::None) {
        draw_arrow(painter, end, diff.normalized(), stroke.color, arrow_style);
    }
}

fn draw_bezier_curve(
    painter: &egui::Painter,
    start: egui::Pos2,
    control1: egui::Pos2,
    control2: egui::Pos2,
    end: egui::Pos2,
    stroke: egui::Stroke,
) {
    let segments = 20;
    let mut points = Vec::with_capacity(segments + 1);
    
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let point = cubic_bezier(start, control1, control2, end, t);
        points.push(point);
    }
    
    for i in 0..segments {
        painter.line_segment([points[i], points[i + 1]], stroke);
    }
}

fn cubic_bezier(p0: egui::Pos2, p1: egui::Pos2, p2: egui::Pos2, p3: egui::Pos2, t: f32) -> egui::Pos2 {
    let u = 1.0 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;
    
    let mut point = p0.to_vec2() * uuu;
    point += p1.to_vec2() * (3.0 * uu * t);
    point += p2.to_vec2() * (3.0 * u * tt);
    point += p3.to_vec2() * ttt;
    
    egui::pos2(point.x, point.y)
}

fn draw_arrow(
    painter: &egui::Painter,
    tip: egui::Pos2,
    direction: egui::Vec2,
    color: egui::Color32,
    style: ArrowStyle,
) {
    let arrow_size = match style {
        ArrowStyle::Simple => 10.0,
        ArrowStyle::Double => 12.0,
        ArrowStyle::Dashed => 8.0,
        ArrowStyle::None => return,
    };
    
    let back = tip - direction * arrow_size;
    let side = direction.rot90() * arrow_size * 0.5;
    
    match style {
        ArrowStyle::Simple | ArrowStyle::Dashed => {
            let triangle = vec![
                tip,
                back + side,
                back - side,
            ];
            
            painter.add(egui::epaint::PathShape::convex_polygon(
                triangle,
                color,
                egui::Stroke::new(1.0, color),
            ));
        },
        ArrowStyle::Double => {
            // Draw two triangular arrows
            let triangle1 = vec![
                tip,
                back + side * 0.7,
                back - side * 0.7,
            ];
            
            let back2 = tip - direction * arrow_size * 1.6;
            let triangle2 = vec![
                back,
                back2 + side * 0.7,
                back2 - side * 0.7,
            ];
            
            painter.add(egui::epaint::PathShape::convex_polygon(
                triangle1,
                color,
                egui::Stroke::new(1.0, color),
            ));
            
            painter.add(egui::epaint::PathShape::convex_polygon(
                triangle2,
                color.linear_multiply(0.8),
                egui::Stroke::new(1.0, color),
            ));
        },
        ArrowStyle::None => {},
    }
}

// Keep existing functions for backward compatibility
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
