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
    
    // Simple force-directed layout
    // For a real implementation, you'd use a proper force-directed algorithm
    
    // Start with a grid layout
    let mut x = -300.0 * zoom;
    let mut y = -200.0 * zoom;
    let file_spacing = 200.0 * zoom;
    
    for (i, file) in project.files.iter().enumerate() {
        let pos = center + egui::vec2(x, y);
        file_positions.insert(file.clone(), pos);
        
        // Move to next position
        x += file_spacing;
        if (i + 1) % 3 == 0 {
            x = -300.0 * zoom;
            y += file_spacing;
        }
    }
    
    // Position code elements with some randomization
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            // Pseudo-random but deterministic positions
            let hash = element.id.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
            let angle = (hash % 628) as f32 / 100.0;  // 0 to 2π
            let distance = 50.0 + (hash % 50) as f32;
            
            let x = file_pos.x + distance * zoom * angle.cos();
            let y = file_pos.y + distance * zoom * angle.sin();
            
            element_positions.insert(element.id.clone(), egui::pos2(x, y));
        }
    }
    
    (file_positions, element_positions)
}
