use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    #[allow(dead_code)]
    Grid,
    #[allow(dead_code)]
    Circular,
    #[allow(dead_code)]
    Tree,
    ForceDirected,
    #[allow(dead_code)]
    Hierarchical,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::ForceDirected
    }
}

#[derive(Debug, Clone)]
pub struct VisualizationState {
    pub zoom: f32,
    pub pan_offset: egui::Vec2,
    pub dragging: bool,
    pub last_pointer_pos: Option<egui::Pos2>,
    pub selected_element: Option<String>,
    pub layout_type: LayoutType,
    pub show_all_relationships: bool,
    pub animation_progress: f32,
    pub show_labels: bool, // Add this new field to control label visibility
}

impl Default for VisualizationState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            dragging: false,
            last_pointer_pos: None,
            selected_element: None,
            layout_type: LayoutType::default(),
            show_all_relationships: true,
            animation_progress: 0.0,
            show_labels: true,
        }
    }
}

impl VisualizationState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    
    // Simple animation function that increases the progress over time
    pub fn update_animation(&mut self, ctx: &egui::Context) {
        // Advance animation by a small amount each frame
        self.animation_progress += 0.01;
        
        // Wrap around when we reach 1.0
        if self.animation_progress >= 1.0 {
            self.animation_progress = 0.0;
        }
        
        // Request a repaint to ensure smooth animation
        ctx.request_repaint();
    }
    
    // UI controls for the visualization settings
    #[allow(dead_code)]
    pub fn ui_controls(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Layout:");
            ui.selectable_value(&mut self.layout_type, LayoutType::ForceDirected, "Force Directed");
            ui.selectable_value(&mut self.layout_type, LayoutType::Grid, "Grid");
            ui.selectable_value(&mut self.layout_type, LayoutType::Circular, "Circular");
            ui.selectable_value(&mut self.layout_type, LayoutType::Tree, "Tree");
            ui.selectable_value(&mut self.layout_type, LayoutType::Hierarchical, "Hierarchical");
        });
        
        ui.horizontal(|ui| {
            ui.checkbox(&mut self.show_all_relationships, "Show All Relationships");
            ui.separator();
            ui.checkbox(&mut self.show_labels, "Show Labels");
            
            ui.separator();
            if ui.button("Reset View").clicked() {
                self.zoom = 1.0;
                self.pan_offset = egui::Vec2::ZERO;
            }
        });
        
        ui.horizontal(|ui| {
            ui.label(format!("Zoom: {:.1}x", self.zoom));
            if ui.button("-").clicked() {
                self.zoom = (self.zoom - 0.1).max(0.1);
            }
            if ui.button("+").clicked() {
                self.zoom = (self.zoom + 0.1).min(5.0);
            }
        });
    }
    
    #[allow(dead_code)]
    pub fn should_draw_labels(&self) -> bool {
        self.show_labels
    }
}
