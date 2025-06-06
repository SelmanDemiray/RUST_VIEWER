use eframe::egui;
use std::collections::HashMap;
use crate::{
    project::Project,
    parser::RelationshipType,
};

pub fn draw_relationships(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &HashMap<String, egui::Pos2>,
    selected_element: Option<&String>,
    show_all: bool,
) {
    for relationship in &project.relationships {
        let should_draw = show_all || 
            selected_element == Some(&relationship.source_id) ||
            selected_element == Some(&relationship.target_id);
            
        if !should_draw {
            continue;
        }
        
        if let (Some(source_pos), Some(target_pos)) = (
            element_positions.get(&relationship.source_id),
            element_positions.get(&relationship.target_id),
        ) {
            let color = get_relationship_color(&relationship.relationship_type);
            let thickness = get_relationship_thickness(&relationship.relationship_type);
            
            painter.line_segment(
                [*source_pos, *target_pos],
                egui::Stroke::new(thickness, color),
            );
            
            // Draw arrow head
            draw_arrow_head(painter, *source_pos, *target_pos, color);
        }
    }
}

pub fn get_relationship_color(rel_type: &RelationshipType) -> egui::Color32 {
    match rel_type {
        RelationshipType::Imports => egui::Color32::from_rgb(100, 100, 255),
        RelationshipType::Uses => egui::Color32::from_rgb(100, 255, 100),
        RelationshipType::Extends => egui::Color32::from_rgb(255, 100, 100),
        RelationshipType::Implements => egui::Color32::from_rgb(255, 255, 100),
        RelationshipType::Contains => egui::Color32::from_rgb(255, 150, 100),
        RelationshipType::Calls => egui::Color32::from_rgb(150, 255, 150),
        RelationshipType::Instantiates => egui::Color32::from_rgb(200, 100, 200),
        RelationshipType::References => egui::Color32::from_rgb(150, 150, 255),
        RelationshipType::DependsOn => egui::Color32::from_rgb(255, 200, 100),
        RelationshipType::AssociatedWith => egui::Color32::from_rgb(200, 200, 200),
    }
}

pub fn get_relationship_thickness(rel_type: &RelationshipType) -> f32 {
    match rel_type {
        RelationshipType::Imports => 1.5,
        RelationshipType::Uses => 1.0,
        RelationshipType::Extends => 2.5,
        RelationshipType::Implements => 2.0,
        RelationshipType::Contains => 1.5,
        RelationshipType::Calls => 1.0,
        RelationshipType::Instantiates => 2.0,
        RelationshipType::References => 1.0,
        RelationshipType::DependsOn => 1.5,
        RelationshipType::AssociatedWith => 1.0,
    }
}

fn draw_arrow_head(
    painter: &egui::Painter,
    start: egui::Pos2,
    end: egui::Pos2,
    color: egui::Color32,
) {
    let direction = (end - start).normalized();
    let arrow_length = 10.0;
    let arrow_angle = std::f32::consts::PI / 6.0; // 30 degrees
    
    let arrow_left = end - arrow_length * egui::Vec2::new(
        direction.x * arrow_angle.cos() - direction.y * arrow_angle.sin(),
        direction.x * arrow_angle.sin() + direction.y * arrow_angle.cos(),
    );
    
    let arrow_right = end - arrow_length * egui::Vec2::new(
        direction.x * arrow_angle.cos() + direction.y * arrow_angle.sin(),
        -direction.x * arrow_angle.sin() + direction.y * arrow_angle.cos(),
    );
    
    painter.line_segment([end, arrow_left], egui::Stroke::new(1.0, color));
    painter.line_segment([end, arrow_right], egui::Stroke::new(1.0, color));
}
