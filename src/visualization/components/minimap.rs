use eframe::egui;
use crate::project::Project;
use crate::visualization::VisualizationState;
use std::collections::HashMap;

#[allow(dead_code)]
// Keep the original function but rename it to avoid conflicts
pub fn draw_minimap_original(ui: &mut egui::Ui, parent_rect: egui::Rect, project: &Project, _state: &VisualizationState) {
    // Only draw minimap if there are elements to display
    if project.elements.is_empty() {
        return;
    }
    
    // Size and position for the minimap
    let minimap_size = egui::vec2(150.0, 100.0);
    let minimap_pos = egui::pos2(
        parent_rect.right() - minimap_size.x - 10.0,
        parent_rect.bottom() - minimap_size.y - 10.0
    );
    let minimap_rect = egui::Rect::from_min_size(minimap_pos, minimap_size);
    
    // Draw minimap background
    ui.painter().rect_filled(
        minimap_rect, 
        5.0, 
        egui::Color32::from_rgba_unmultiplied(40, 40, 40, 200)
    );
    
    // Draw border
    ui.painter().rect_stroke(
        minimap_rect,
        5.0,
        egui::Stroke::new(1.0, egui::Color32::from_gray(100))
    );
    
    // Draw dots for each element (simplified representation)
    for element in &project.elements {
        // In a real implementation, you'd calculate normalized positions
        // based on the actual layout in the main view
        let normalized_x = (element.file_path.len() as f32 % 10.0) / 10.0;
        let normalized_y = (element.name.len() as f32 % 10.0) / 10.0;
        
        let dot_pos = egui::pos2(
            minimap_rect.min.x + normalized_x * minimap_rect.width(),
            minimap_rect.min.y + normalized_y * minimap_rect.height()
        );
        
        ui.painter().circle_filled(
            dot_pos,
            3.0,
            egui::Color32::from_gray(200)
        );
    }
    
    // Draw a rectangle representing the current view area
    // In a real implementation, you'd calculate this based on the actual view position and size
    let view_rect = egui::Rect::from_min_size(
        egui::pos2(
            minimap_rect.min.x + minimap_rect.width() * 0.2,
            minimap_rect.min.y + minimap_rect.height() * 0.2
        ),
        egui::vec2(
            minimap_rect.width() * 0.4,
            minimap_rect.height() * 0.4
        )
    );
    
    ui.painter().rect_stroke(
        view_rect,
        0.0,
        egui::Stroke::new(1.0, egui::Color32::WHITE)
    );
}

// Add the new function with the signature expected by the renderer
pub fn draw_minimap(
    ui: &mut egui::Ui, 
    rect: egui::Rect, 
    file_positions: &HashMap<String, egui::Pos2>, 
    element_positions: &HashMap<String, egui::Pos2>,
    state: &VisualizationState
) {
    // Size and position for the minimap
    let minimap_size = egui::vec2(150.0, 100.0);
    let minimap_pos = egui::pos2(
        rect.right() - minimap_size.x - 10.0,
        rect.bottom() - minimap_size.y - 10.0
    );
    let minimap_rect = egui::Rect::from_min_size(minimap_pos, minimap_size);
    
    // Draw minimap background
    ui.painter().rect_filled(
        minimap_rect, 
        5.0, 
        egui::Color32::from_rgba_unmultiplied(40, 40, 40, 200)
    );
    
    // Draw border
    ui.painter().rect_stroke(
        minimap_rect,
        5.0,
        egui::Stroke::new(1.0, egui::Color32::from_gray(100))
    );
    
    // Draw dots for files
    for (_file, pos) in file_positions {
        // Calculate normalized position in minimap
        let normalized_x = (pos.x - rect.left()) / rect.width();
        let normalized_y = (pos.y - rect.top()) / rect.height();
        
        let dot_pos = egui::pos2(
            minimap_rect.min.x + normalized_x * minimap_rect.width(),
            minimap_rect.min.y + normalized_y * minimap_rect.height()
        );
        
        ui.painter().circle_filled(
            dot_pos,
            4.0,
            egui::Color32::from_rgb(70, 70, 120)
        );
    }
    
    // Draw dots for elements
    for (_id, pos) in element_positions {
        // Calculate normalized position in minimap
        let normalized_x = (pos.x - rect.left()) / rect.width();
        let normalized_y = (pos.y - rect.top()) / rect.height();
        
        let dot_pos = egui::pos2(
            minimap_rect.min.x + normalized_x * minimap_rect.width(),
            minimap_rect.min.y + normalized_y * minimap_rect.height()
        );
        
        ui.painter().circle_filled(
            dot_pos,
            2.0,
            egui::Color32::from_gray(200)
        );
    }
    
    // Draw view area rectangle
    let view_min_normalized = egui::vec2(
        (rect.min.x - rect.min.x) / rect.width() - state.pan_offset.x / rect.width() / state.zoom,
        (rect.min.y - rect.min.y) / rect.height() - state.pan_offset.y / rect.height() / state.zoom
    );
    
    let view_max_normalized = egui::vec2(
        view_min_normalized.x + 1.0 / state.zoom,
        view_min_normalized.y + 1.0 / state.zoom
    );
    
    let view_rect = egui::Rect::from_min_max(
        egui::pos2(
            minimap_rect.min.x + view_min_normalized.x * minimap_rect.width(),
            minimap_rect.min.y + view_min_normalized.y * minimap_rect.height()
        ),
        egui::pos2(
            minimap_rect.min.x + view_max_normalized.x * minimap_rect.width(),
            minimap_rect.min.y + view_max_normalized.y * minimap_rect.height()
        )
    );
    
    ui.painter().rect_stroke(
        view_rect,
        0.0,
        egui::Stroke::new(1.0, egui::Color32::WHITE)
    );
}
