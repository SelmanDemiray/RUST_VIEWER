mod state;
mod renderer;
mod components;
mod layout;

pub use state::VisualizationState;
pub use renderer::{VisualizationRenderer, Node, Edge};

use eframe::egui;
use crate::project::Project;

// Re-export the render function for backward compatibility
pub fn render_visualization(ui: &mut egui::Ui, project: &Project, state: &mut VisualizationState) {
    // Set some default values if they're not set
    if state.zoom == 0.0 {
        state.zoom = 1.0;
    }
    
    // Add a visualization area with panning and zooming
    let frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(30, 30, 30));
    
    frame.show(ui, |ui| {
        // Use available space
        let available_rect = ui.available_rect_before_wrap();
        
        // Update animation if needed
        state.update_animation(ui.ctx());
        
        // Calculate layout positions
        let (file_positions, element_positions) = layout::calculate_positions(
            project,
            &state.layout_type,
            state.zoom,
            available_rect.center(),
            state.animation_progress
        );
        
        // Handle interactions (drag, zoom, etc.)
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
        
        // Handle dragging for panning
        if response.dragged() {
            state.dragging = true;
            if let Some(_pointer_pos) = state.last_pointer_pos {
                let delta = response.drag_delta();
                state.pan_offset += delta;
            }
            state.last_pointer_pos = Some(response.hover_pos().unwrap_or_default());
        } else {
            state.dragging = false;
            state.last_pointer_pos = None;
        }
        
        // Handle zooming with scroll
        if response.hovered() {
            ui.input(|i| {
                let scroll_delta = i.scroll_delta.y;
                if scroll_delta != 0.0 {
                    let old_zoom = state.zoom;
                    state.zoom *= 1.0 + scroll_delta * 0.001;
                    state.zoom = state.zoom.clamp(0.1, 5.0);
                    
                    // Adjust pan to keep the point under cursor stationary
                    if let Some(mouse_pos) = i.pointer.hover_pos() {
                        let zoom_center = mouse_pos;
                        let zoom_diff = state.zoom / old_zoom;
                        // Convert to Vec2 operations to avoid type mismatch
                        let center_to_mouse = zoom_center - available_rect.center();
                        let offset_delta = center_to_mouse - center_to_mouse * zoom_diff;
                        state.pan_offset += offset_delta;
                    }
                }
            });
        }
        
        // First do operations that need mutable ui
        components::elements::draw(
            ui,
            project,
            state,
            &element_positions,
            &response
        );
        
        // Then get painter for immutable operations
        let painter = ui.painter();
        
        // Draw relationships
        components::relationships::draw_relationships(
            project,
            painter,
            &element_positions,
            state.selected_element.as_ref(),
            state.show_all_relationships
        );
        
        // Draw minimap
        components::minimap::draw_minimap(
            ui,
            available_rect,
            &file_positions,
            &element_positions,
            state
        );
        
        // Draw status bar
        components::status_bar::draw_status(ui, project, state);
    });
}

#[allow(dead_code)]
pub struct VisualizationPanel {
    state: VisualizationState,
    renderer: VisualizationRenderer,
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl VisualizationPanel {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: VisualizationState::new(),
            renderer: VisualizationRenderer::new(),
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    #[allow(dead_code)]
    pub fn ui(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            egui::SidePanel::left("viz_controls")
                .resizable(true)
                .min_width(200.0)
                .show_inside(ui, |ui| {
                    self.state.ui_controls(ui);
                });
            
            egui::CentralPanel::default().show_inside(ui, |ui| {
                // Add a frame for better visuals
                egui::Frame::canvas(ui.style()).show(ui, |ui| {
                    self.renderer.render(ui, &self.state, &self.nodes, &self.edges);
                });
            });
        });
        
        // Update animation if needed
        self.state.update_animation(ctx);
    }
    
    #[allow(dead_code)]
    pub fn set_data(&mut self, nodes: Vec<Node>, edges: Vec<Edge>) {
        self.nodes = nodes;
        self.edges = edges;
        // Reset animation when new data is loaded
        self.state.animation_progress = 0.0;
    }
}
