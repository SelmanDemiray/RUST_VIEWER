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
    // Reset layout when project changes significantly
    static mut LAST_PROJECT_HASH: u64 = 0;
    let current_hash = calculate_project_hash(project);
    unsafe {
        if LAST_PROJECT_HASH != current_hash {
            layout::reset_layout();
            LAST_PROJECT_HASH = current_hash;
            state.animation_progress = 0.0;
        }
    }
    
    // Ensure zoom is reasonable
    if state.zoom <= 0.0 || state.zoom.is_nan() {
        state.zoom = 1.0;
    }
    
    // Add a visualization area with better styling
    let frame = egui::Frame::none()
        .fill(egui::Color32::from_rgb(20, 25, 30))
        .inner_margin(egui::style::Margin::same(0.0));
    
    frame.show(ui, |ui| {
        let available_rect = ui.available_rect_before_wrap();
        
        // Smoother animation updates
        state.update_animation(ui.ctx());
        
        // Calculate layout positions with improved stability
        let view_center = available_rect.center() + state.pan_offset;
        let (file_positions, element_positions) = layout::calculate_positions(
            project,
            &state.layout_type,
            state.zoom,
            view_center,
            state.animation_progress
        );
        
        // Handle interactions with improved responsiveness
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());
        
        // Smooth panning
        if response.dragged() {
            let delta = response.drag_delta();
            state.pan_offset += delta;
            state.dragging = true;
        } else {
            state.dragging = false;
        }
        
        // Improved zooming
        if response.hovered() {
            ui.input(|i| {
                let scroll_delta = i.scroll_delta.y;
                if scroll_delta.abs() > 0.1 {
                    let old_zoom = state.zoom;
                    let zoom_factor = 1.0 + scroll_delta * 0.001;
                    state.zoom = (state.zoom * zoom_factor).clamp(0.3, 2.5);
                    
                    // Zoom towards mouse position
                    if let Some(mouse_pos) = i.pointer.hover_pos() {
                        let zoom_center = mouse_pos - available_rect.center();
                        let zoom_change = state.zoom / old_zoom - 1.0;
                        state.pan_offset -= zoom_center * zoom_change * 0.5;
                    }
                }
            });
        }
        
        // Draw components in proper order
        components::elements::draw(
            ui,
            project,
            state,
            &element_positions,
            &response
        );
        
        let painter = ui.painter();
        
        components::relationships::draw_relationships(
            project,
            painter,
            &element_positions,
            state.selected_element.as_ref(),
            state.show_all_relationships
        );
        
        components::minimap::draw_minimap(
            ui,
            available_rect,
            &file_positions,
            &element_positions,
            state
        );
        
        components::status_bar::draw_status(ui, project, state);
        
        // Improved empty state
        if project.elements.is_empty() {
            draw_improved_empty_state(ui, available_rect);
        }
    });
}

fn calculate_project_hash(project: &Project) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    project.files.len().hash(&mut hasher);
    project.elements.len().hash(&mut hasher);
    project.relationships.len().hash(&mut hasher);
    hasher.finish()
}

fn draw_improved_empty_state(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    
    // Subtle overlay
    painter.rect_filled(
        rect,
        0.0,
        egui::Color32::from_rgba_unmultiplied(0, 0, 0, 50)
    );
    
    // Main message
    let center = rect.center();
    painter.text(
        center + egui::vec2(0.0, -20.0),
        egui::Align2::CENTER_CENTER,
        "Welcome to Rust Code Visualizer",
        egui::FontId::proportional(24.0),
        egui::Color32::from_gray(220)
    );
    
    painter.text(
        center + egui::vec2(0.0, 10.0),
        egui::Align2::CENTER_CENTER,
        "Click File → Open Project to visualize your Rust code",
        egui::FontId::proportional(16.0),
        egui::Color32::from_gray(180)
    );
    
    // Decorative elements
    for i in 0..6 {
        let angle = (i as f32 / 6.0) * std::f32::consts::TAU;
        let radius = 100.0;
        let pos = center + egui::vec2(
            radius * angle.cos(),
            radius * angle.sin()
        );
        
        painter.circle_filled(
            pos,
            8.0,
            egui::Color32::from_rgba_unmultiplied(100, 150, 200, 100)
        );
    }
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
