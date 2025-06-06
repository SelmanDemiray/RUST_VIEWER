use eframe::egui;
use crate::visualization::state::VisualizationState;
use crate::parser::types::ElementType;

pub fn render_visualization(ui: &mut egui::Ui, state: &mut VisualizationState) {
    // Create a painter for drawing
    let (response, painter) = ui.allocate_painter(
        ui.available_size(),
        egui::Sense::click_and_drag()
    );
    
    let rect = response.rect;
    
    // Draw background
    painter.rect_filled(rect, 0.0, egui::Color32::from_gray(20));
    
    // If no nodes, show helpful message
    if state.nodes.is_empty() {
        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "No code elements found.\nMake sure you've loaded a Rust project with .rs files.",
            egui::FontId::proportional(16.0),
            egui::Color32::WHITE,
        );
        return;
    }
    
    // Calculate layout if positions are not set
    if state.nodes.iter().all(|n| n.position == egui::Pos2::ZERO) {
        calculate_initial_layout(state, rect);
    }
    
    // Handle mouse interactions
    if response.dragged() {
        state.pan_offset += response.drag_delta();
    }
    
    if let Some(hover_pos) = response.hover_pos() {
        let scroll_delta = ui.input(|i| i.scroll_delta);
        if scroll_delta.y != 0.0 {
            let zoom_factor = 1.0 + scroll_delta.y * 0.001;
            state.zoom = (state.zoom * zoom_factor).clamp(0.1, 5.0);
        }
        
        // Check for node clicks
        if response.clicked() {
            for node in &state.nodes {
                let screen_pos = rect.center() + node.position.to_vec2() * state.zoom + state.pan_offset;
                let radius = node.size.x / 2.0 * state.zoom;
                
                if (hover_pos - screen_pos).length() <= radius {
                    state.selected_element = Some(node.element.id.clone());
                    break;
                }
            }
        }
    }
    
    // Draw edges first (so they appear behind nodes)
    for edge in &state.edges {
        if let (Some(source_node), Some(target_node)) = (
            state.nodes.iter().find(|n| n.element.id == edge.source),
            state.nodes.iter().find(|n| n.element.id == edge.target)
        ) {
            let start_pos = rect.center() + source_node.position.to_vec2() * state.zoom + state.pan_offset;
            let end_pos = rect.center() + target_node.position.to_vec2() * state.zoom + state.pan_offset;
            
            let color = match edge.relationship.relationship_type {
                crate::parser::RelationshipType::Contains => egui::Color32::from_rgb(100, 100, 255),
                crate::parser::RelationshipType::Uses => egui::Color32::from_rgb(100, 255, 100),
                _ => egui::Color32::GRAY,
            };
            
            painter.line_segment(
                [start_pos, end_pos],
                egui::Stroke::new(1.0 * state.zoom, color)
            );
        }
    }
    
    // Draw nodes
    for node in &state.nodes {
        let screen_pos = rect.center() + node.position.to_vec2() * state.zoom + state.pan_offset;
        
        let color = match node.element.element_type {
            ElementType::Function => egui::Color32::from_rgb(100, 200, 255),
            ElementType::Struct => egui::Color32::from_rgb(100, 255, 100),
            ElementType::Enum => egui::Color32::from_rgb(255, 255, 100),
            ElementType::Trait => egui::Color32::from_rgb(255, 100, 100),
            ElementType::Module => egui::Color32::from_rgb(200, 200, 200),
            ElementType::Impl => egui::Color32::from_rgb(200, 150, 255),
        };
        
        let radius = node.size.x / 2.0 * state.zoom;
        
        // Draw node circle
        painter.circle_filled(screen_pos, radius, color);
        
        // Highlight selected node
        if state.selected_element.as_ref() == Some(&node.element.id) {
            painter.circle_stroke(
                screen_pos, 
                radius + 2.0, 
                egui::Stroke::new(2.0, egui::Color32::YELLOW)
            );
        } else {
            painter.circle_stroke(
                screen_pos, 
                radius, 
                egui::Stroke::new(1.0, egui::Color32::WHITE)
            );
        }
        
        // Draw node label if zoomed in enough
        if state.zoom > 0.5 {
            painter.text(
                screen_pos + egui::vec2(0.0, radius + 10.0 * state.zoom),
                egui::Align2::CENTER_TOP,
                &node.element.name,
                egui::FontId::proportional(12.0 * state.zoom),
                egui::Color32::WHITE,
            );
        }
    }
    
    // Draw info panel
    draw_info_panel(ui, state, rect);
}

fn calculate_initial_layout(state: &mut VisualizationState, rect: egui::Rect) {
    let node_count = state.nodes.len();
    if node_count == 0 {
        return;
    }
    
    // Simple circular layout
    let radius = rect.width().min(rect.height()) * 0.3;
    
    for (i, node) in state.nodes.iter_mut().enumerate() {
        let angle = if node_count > 1 {
            2.0 * std::f32::consts::PI * i as f32 / node_count as f32
        } else {
            0.0
        };
        
        node.position = egui::pos2(
            radius * angle.cos(),
            radius * angle.sin()
        );
    }
}

fn draw_info_panel(ui: &mut egui::Ui, state: &VisualizationState, rect: egui::Rect) {
    let panel_width = 200.0;
    let panel_rect = egui::Rect::from_min_size(
        rect.max - egui::vec2(panel_width + 10.0, rect.height() - 10.0),
        egui::vec2(panel_width, rect.height() - 20.0)
    );
    
    let painter = ui.painter();
    
    // Draw panel background
    painter.rect_filled(
        panel_rect,
        5.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 200)
    );
    painter.rect_stroke(
        panel_rect,
        5.0,
        egui::Stroke::new(1.0, egui::Color32::GRAY)
    );
    
    // Draw panel content
    ui.allocate_ui_at_rect(panel_rect.shrink(10.0), |ui| {
        ui.vertical(|ui| {
            ui.label(egui::RichText::new("Project Info").heading().color(egui::Color32::WHITE));
            ui.separator();
            
            ui.label(egui::RichText::new(format!("Elements: {}", state.nodes.len())).color(egui::Color32::WHITE));
            ui.label(egui::RichText::new(format!("Relationships: {}", state.edges.len())).color(egui::Color32::WHITE));
            ui.label(egui::RichText::new(format!("Zoom: {:.1}%", state.zoom * 100.0)).color(egui::Color32::WHITE));
            
            if let Some(ref selected) = state.selected_element {
                ui.separator();
                ui.label(egui::RichText::new("Selected:").color(egui::Color32::YELLOW));
                if let Some(node) = state.nodes.iter().find(|n| &n.element.id == selected) {
                    ui.label(egui::RichText::new(&node.element.name).color(egui::Color32::WHITE));
                    ui.label(egui::RichText::new(format!("{:?}", node.element.element_type)).color(egui::Color32::GRAY));
                    ui.label(egui::RichText::new(&node.element.file_path).color(egui::Color32::GRAY));
                }
            }
        });
    });
}
