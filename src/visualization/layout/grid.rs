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
    
    // Position files in a grid
    let mut x = -300.0 * zoom;
    let mut y = -200.0 * zoom;
    let file_spacing_x = 200.0 * zoom;
    let file_spacing_y = 150.0 * zoom;
    
    for (i, file) in project.files.iter().enumerate() {
        let pos = center + egui::vec2(x, y);
        file_positions.insert(file.clone(), pos);
        
        // Move to next position
        x += file_spacing_x;
        if (i + 1) % 3 == 0 {
            x = -300.0 * zoom;
            y += file_spacing_y;
        }
    }
    
    // Position code elements below their files
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            let element_offset = egui::vec2(
                (element.id.len() as f32 % 3.0 - 1.0) * 50.0 * zoom,
                70.0 * zoom
            );
            element_positions.insert(element.id.clone(), *file_pos + element_offset);
        }
    }
    
    (file_positions, element_positions)
}
