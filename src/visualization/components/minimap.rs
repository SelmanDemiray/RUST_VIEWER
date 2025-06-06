use eframe::egui;
use std::collections::HashMap;
use crate::visualization::state::VisualizationState;

pub fn draw_minimap(
    ui: &mut egui::Ui,
    main_rect: egui::Rect,
    _file_positions: &HashMap<String, egui::Pos2>,
    element_positions: &HashMap<String, egui::Pos2>,
    state: &VisualizationState,
) {
    let minimap_size = egui::vec2(150.0, 100.0);
    let minimap_rect = egui::Rect::from_min_size(
        main_rect.max - minimap_size - egui::vec2(10.0, 10.0),
        minimap_size,
    );
    
    let painter = ui.painter();
    
    // Draw minimap background
    painter.rect_filled(
        minimap_rect,
        5.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 150),
    );
    painter.rect_stroke(
        minimap_rect,
        5.0,
        egui::Stroke::new(1.0, egui::Color32::GRAY),
    );
    
    // Draw elements as dots in minimap
    if !element_positions.is_empty() {
        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        
        for pos in element_positions.values() {
            min_x = min_x.min(pos.x);
            max_x = max_x.max(pos.x);
            min_y = min_y.min(pos.y);
            max_y = max_y.max(pos.y);
        }
        
        let world_width = max_x - min_x;
        let world_height = max_y - min_y;
        
        if world_width > 0.0 && world_height > 0.0 {
            for pos in element_positions.values() {
                let normalized_x = (pos.x - min_x) / world_width;
                let normalized_y = (pos.y - min_y) / world_height;
                
                let minimap_pos = minimap_rect.min + egui::vec2(
                    normalized_x * minimap_rect.width(),
                    normalized_y * minimap_rect.height(),
                );
                
                painter.circle_filled(minimap_pos, 2.0, egui::Color32::WHITE);
            }
        }
        
        // Draw viewport indicator
        let viewport_rect = egui::Rect::from_center_size(
            minimap_rect.center() - state.pan_offset * 0.1,
            minimap_size * 0.3,
        );
        
        painter.rect_stroke(
            viewport_rect,
            2.0,
            egui::Stroke::new(1.0, egui::Color32::YELLOW),
        );
    }
}
