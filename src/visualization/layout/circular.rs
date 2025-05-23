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
    
    // Position files in a circle
    let file_count = project.files.len() as f32;
    let radius = 250.0 * zoom;
    
    for (i, file) in project.files.iter().enumerate() {
        let angle = (i as f32 / file_count) * std::f32::consts::TAU;
        let x = radius * angle.cos();
        let y = radius * angle.sin();
        let pos = center + egui::vec2(x, y);
        file_positions.insert(file.clone(), pos);
    }
    
    // Position elements in smaller circles around their files
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            let element_count = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .count() as f32;
            
            let element_index = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .position(|e| e.id == element.id)
                .unwrap_or(0) as f32;
            
            let angle = (element_index / element_count.max(1.0)) * std::f32::consts::TAU;
            let small_radius = 60.0 * zoom;
            let x = small_radius * angle.cos();
            let y = small_radius * angle.sin();
            
            element_positions.insert(element.id.clone(), *file_pos + egui::vec2(x, y));
        }
    }
    
    (file_positions, element_positions)
}
