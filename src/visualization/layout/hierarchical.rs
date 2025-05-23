use crate::project::Project;
use eframe::egui;
use std::collections::HashMap;

pub fn calculate(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    _animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();
    
    // Position files in a hierarchical structure
    let max_depth = 4;
    let level_height = 150.0 * zoom;
    let node_width = 180.0 * zoom;
    
    // Start with root at the top center
    let root_y = center.y - (max_depth as f32 * level_height) / 2.0;
    
    // Organize files by hierarchy level (using file path depth as a proxy for hierarchy)
    let mut files_by_level: Vec<Vec<&String>> = vec![Vec::new(); max_depth];
    
    for file in &project.files {
        let depth = file.split('/').count().min(max_depth) - 1;
        files_by_level[depth].push(file);
    }
    
    // Position files level by level
    for (level, files) in files_by_level.iter().enumerate() {
        let y = root_y + (level as f32 * level_height);
        let width = node_width * files.len() as f32;
        let start_x = center.x - width / 2.0 + node_width / 2.0;
        
        for (i, file) in files.iter().enumerate() {
            let x = start_x + i as f32 * node_width;
            file_positions.insert((*file).clone(), egui::pos2(x, y));
        }
    }
    
    // Position elements under their parent files with radial arrangement
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            // Get all elements in the same file
            let elements_in_file = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .count();
            
            let element_index = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .position(|e| e.id == element.id)
                .unwrap_or(0);
            
            // Use a small semi-circle below the file
            let angle_step = std::f32::consts::PI / (elements_in_file as f32);
            let angle = std::f32::consts::PI / 2.0 + angle_step * (element_index as f32 + 0.5);
            
            let radius = 70.0 * zoom;
            let x = file_pos.x + radius * angle.cos();
            let y = file_pos.y + radius * angle.sin();
            
            element_positions.insert(element.id.clone(), egui::pos2(x, y));
        }
    }
    
    (file_positions, element_positions)
}
