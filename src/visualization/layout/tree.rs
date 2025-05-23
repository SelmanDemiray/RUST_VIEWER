use eframe::egui;
use std::collections::HashMap;

use crate::project::Project;

pub fn calculate(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    _animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();
    
    // Position files in a hierarchical tree
    let max_depth = 3;
    let width = 600.0 * zoom;
    let height = 500.0 * zoom;
    
    // Root at the top
    if let Some(first_file) = project.files.first() {
        file_positions.insert(
            first_file.clone(),
            center + egui::vec2(0.0, -height/2.0)
        );
    }
    
    // Other files in levels below
    for (i, file) in project.files.iter().skip(1).enumerate() {
        let depth = (i / 3) + 1;
        if depth >= max_depth { continue; }
        
        let items_at_level = 3.min(project.files.len() - 1 - (i / 3) * 3);
        let position_in_level = i % 3;
        
        let x = width * ((position_in_level as f32 + 0.5) / items_at_level as f32 - 0.5);
        let y = (depth as f32 * height) / max_depth as f32 - height/2.0;
        
        file_positions.insert(file.clone(), center + egui::vec2(x, y));
    }
    
    // Position elements as leaf nodes
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            let elements_in_file = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .count();
            
            let element_index = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .position(|e| e.id == element.id)
                .unwrap_or(0);
            
            // Arrange elements in a row below their file
            let total_width = 150.0 * zoom * elements_in_file as f32;
            let element_spacing = total_width / elements_in_file.max(1) as f32;
            let x_offset = -total_width / 2.0 + element_spacing / 2.0 + element_index as f32 * element_spacing;
            let y_offset = 60.0 * zoom;
            
            element_positions.insert(
                element.id.clone(), 
                *file_pos + egui::vec2(x_offset, y_offset)
            );
        }
    }
    
    (file_positions, element_positions)
}
