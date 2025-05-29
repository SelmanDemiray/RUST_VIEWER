use eframe::egui;

use crate::{
    project::Project,
    visualization::{
        components,
        layout,
        state::VisualizationState,
    },
};

#[allow(dead_code)]
pub struct VisualizationRenderer {
    // Add renderer-specific fields here
    state: VisualizationState,
}

impl VisualizationRenderer {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            state: VisualizationState::new(),
        }
    }

    #[allow(dead_code)]
    pub fn render(&mut self, ui: &mut egui::Ui, state: &VisualizationState, _nodes: &[Node], _edges: &[Edge]) {
        // Create a simple project from nodes and edges for backward compatibility
        let project = Project::default();
        // Use nodes and edges to populate the project
        // For a real implementation, you'd need to convert between these formats
        
        let mut state_copy = state.clone();
        render(ui, &project, &mut state_copy, _nodes, _edges);
        
        // Update the internal state with any changes
        self.state = state_copy;
    }
}

#[allow(dead_code)]
pub struct Node {
    pub id: String,
    pub label: String,
    pub position: egui::Pos2,
    pub color: egui::Color32,
    pub size: f32,
    pub is_selected: bool,
}

#[allow(dead_code)]
pub struct Edge {
    pub from: String,
    pub to: String,
    pub color: egui::Color32,
    pub thickness: f32,
}

#[allow(dead_code)]
pub fn render(ui: &mut egui::Ui, project: &Project, state: &mut VisualizationState, _nodes: &[Node], _edges: &[Edge]) {
    // Set some default values if they're not set
    if state.zoom == 0.0 {
        state.zoom = 1.0;
    }

    // Add a visualization area with panning and zooming
    let frame = egui::Frame::none().fill(egui::Color32::from_rgb(30, 30, 30));

    frame.show(ui, |ui| {
        // Use available space
        let available_rect = ui.available_rect_before_wrap();

        // Calculate layout positions
        let (file_positions, element_positions) = layout::calculate_positions(
            project,
            &state.layout_type,
            state.zoom,
            available_rect.center() + state.pan_offset,
            state.animation_progress,
        );

        // Handle interactions (drag, zoom, etc.)
        let response = ui.allocate_rect(available_rect, egui::Sense::click_and_drag());

        // Restructure to avoid borrowing issues
        // First, do operations that need a mutable ui
        components::elements::draw(ui, project, state, &element_positions, &response);
        
        // After mutable operations, get the painter for immutable operations
        let painter = ui.painter();

        // Draw relationships
        components::relationships::draw_relationships(
            project,
            painter,
            &element_positions,
            state.selected_element.as_ref(),
            state.show_all_relationships,
        );

        // Draw minimap
        components::minimap::draw_minimap(
            ui,
            available_rect,
            &file_positions,
            &element_positions,
            state,
        );

        // Draw status bar
        components::status_bar::draw_status(ui, project, state);
    });
}
