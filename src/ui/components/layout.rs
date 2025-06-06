use eframe::egui;
use crate::{
    LayoutSettings,
    visualization::state::{VisualizationState, LayoutType},
};

pub fn render_layout_controls(ui: &mut egui::Ui) {
    ui.heading("Layout Controls");
    ui.separator();
    
    // Create a default layout state if needed
    let mut layout_type = LayoutType::ForceDirected;
    let mut settings = LayoutSettings::default();
    
    render_layout_type_selector(ui, &mut layout_type);
    ui.separator();
    
    render_layout_settings(ui, &mut settings, &layout_type);
    ui.separator();
    
    render_visibility_controls(ui, &mut settings);
    ui.separator();
    
    render_layout_actions(ui);
}

pub fn render_layout_controls_with_state(ui: &mut egui::Ui, state: &mut VisualizationState, settings: &mut LayoutSettings) {
    ui.heading("Layout Controls");
    ui.separator();
    
    render_layout_type_selector(ui, &mut state.layout_type);
    ui.separator();
    
    render_layout_settings(ui, settings, &state.layout_type);
    ui.separator();
    
    render_visibility_controls_with_state(ui, state, settings);
    ui.separator();
    
    render_layout_actions(ui);
}

fn render_layout_type_selector(ui: &mut egui::Ui, layout_type: &mut LayoutType) {
    ui.label("Layout Algorithm:");
    ui.horizontal(|ui| {
        ui.selectable_value(layout_type, LayoutType::ForceDirected, "Force Directed");
        ui.selectable_value(layout_type, LayoutType::Grid, "Grid");
        ui.selectable_value(layout_type, LayoutType::Circular, "Circular");
    });
    ui.horizontal(|ui| {
        ui.selectable_value(layout_type, LayoutType::Tree, "Tree");
        ui.selectable_value(layout_type, LayoutType::Hierarchical, "Hierarchical");
    });
}

fn render_layout_settings(ui: &mut egui::Ui, settings: &mut LayoutSettings, layout_type: &LayoutType) {
    match layout_type {
        LayoutType::ForceDirected => render_force_directed_settings(ui, settings),
        LayoutType::Grid => render_grid_settings(ui),
        LayoutType::Circular => render_circular_settings(ui),
        LayoutType::Tree => render_tree_settings(ui),
        LayoutType::Hierarchical => render_hierarchical_settings(ui),
    }
}

fn render_force_directed_settings(ui: &mut egui::Ui, settings: &mut LayoutSettings) {
    ui.label("Force-Directed Settings:");
    
    ui.add(
        egui::Slider::new(&mut settings.force_strength, 0.01..=1.0)
            .text("Force Strength")
            .step_by(0.01)
    );
    
    ui.add(
        egui::Slider::new(&mut settings.repulsion_strength, 10.0..=500.0)
            .text("Repulsion Strength")
            .step_by(5.0)
    );
    
    ui.add(
        egui::Slider::new(&mut settings.spring_length, 20.0..=150.0)
            .text("Spring Length")
            .step_by(5.0)
    );
    
    ui.add(
        egui::Slider::new(&mut settings.damping, 0.1..=0.99)
            .text("Damping")
            .step_by(0.01)
    );
}

fn render_grid_settings(ui: &mut egui::Ui) {
    ui.label("Grid Settings:");
    ui.label("• Automatically arranges elements in a grid");
    ui.label("• Grid size adapts to element count");
}

fn render_circular_settings(ui: &mut egui::Ui) {
    ui.label("Circular Settings:");
    ui.label("• Arranges files in outer circle");
    ui.label("• Elements orbit around their files");
}

fn render_tree_settings(ui: &mut egui::Ui) {
    ui.label("Tree Settings:");
    ui.label("• Hierarchical tree structure");
    ui.label("• Groups elements by file");
}

fn render_hierarchical_settings(ui: &mut egui::Ui) {
    ui.label("Hierarchical Settings:");
    ui.label("• Groups by element type");
    ui.label("• Layered vertical arrangement");
}

fn render_visibility_controls(ui: &mut egui::Ui, settings: &mut LayoutSettings) {
    ui.label("Visibility Options:");
    ui.checkbox(&mut settings.show_dependencies, "Show Dependencies");
    ui.checkbox(&mut settings.show_functions, "Show Functions");
    ui.checkbox(&mut settings.show_structs, "Show Structs");
}

fn render_visibility_controls_with_state(ui: &mut egui::Ui, state: &mut VisualizationState, settings: &mut LayoutSettings) {
    ui.label("Visibility Options:");
    ui.checkbox(&mut state.show_relationships, "Show Relationships");
    ui.checkbox(&mut state.show_functions, "Show Functions");
    ui.checkbox(&mut state.show_structs, "Show Structs");
    ui.checkbox(&mut settings.show_dependencies, "Show Dependencies");
}

fn render_layout_actions(ui: &mut egui::Ui) {
    ui.label("Actions:");
    ui.horizontal(|ui| {
        if ui.button("Reset Layout").clicked() {
            crate::visualization::layout::reset_layout();
        }
        if ui.button("Center View").clicked() {
            // TODO: Implement center view functionality
        }
    });
    ui.horizontal(|ui| {
        if ui.button("Auto Arrange").clicked() {
            // TODO: Trigger auto arrangement
        }
        if ui.button("Fit to Screen").clicked() {
            // TODO: Implement fit to screen
        }
    });
}

// Additional helper functions for advanced layout controls
pub fn render_zoom_controls(ui: &mut egui::Ui, zoom: &mut f32) {
    ui.label("Zoom:");
    ui.horizontal(|ui| {
        if ui.button("−").clicked() {
            *zoom = (*zoom * 0.8).max(0.1);
        }
        ui.add(
            egui::Slider::new(zoom, 0.1..=3.0)
                .logarithmic(true)
                .show_value(false)
        );
        if ui.button("+").clicked() {
            *zoom = (*zoom * 1.25).min(3.0);
        }
    });
    ui.label(format!("{:.0}%", *zoom * 100.0)); // Fix: dereference zoom
}

pub fn render_filter_controls(ui: &mut egui::Ui, state: &mut VisualizationState) {
    ui.label("Filter Elements:");
    
    egui::CollapsingHeader::new("Element Types")
        .default_open(false)
        .show(ui, |ui| {
            ui.checkbox(&mut state.show_functions, "Functions");
            ui.checkbox(&mut state.show_structs, "Structs");
            // Add more element type filters as needed
        });
}

// Extension for VisualizationState to support label drawing
impl VisualizationState {
    pub fn should_draw_labels(&self) -> bool {
        self.zoom > 0.3 // Only show labels when zoomed in enough
    }
}
