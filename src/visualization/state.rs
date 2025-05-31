use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    ForceDirected,
    Grid,
    Circular,
    Tree,
    Hierarchical,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::ForceDirected
    }
}

#[derive(Clone)]
pub struct VisualizationState {
    pub zoom: f32,
    pub pan_offset: egui::Vec2,
    pub layout_type: LayoutType,
    pub selected_element: Option<String>,
    pub show_all_relationships: bool,
    pub animation_progress: f32,
    pub dragging: bool,
    pub last_pointer_pos: Option<egui::Pos2>,
    pub filter_text: String,
    pub show_labels: bool,
}

impl Default for VisualizationState {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualizationState {
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            layout_type: LayoutType::default(),
            selected_element: None,
            show_all_relationships: false,
            animation_progress: 0.0,
            dragging: false,
            last_pointer_pos: None,
            filter_text: String::new(),
            show_labels: true,
        }
    }
    
    pub fn should_draw_labels(&self) -> bool {
        self.show_labels
    }
    
    pub fn update_animation(&mut self, _ctx: &egui::Context) {
        // Simple animation progress
        self.animation_progress += 0.016; // ~60fps
        if self.animation_progress > 1.0 {
            self.animation_progress = 1.0;
        }
    }
    
    pub fn ui_controls(&mut self, ui: &mut egui::Ui) {
        ui.heading("Visualization Controls");
        ui.separator();
        
        ui.label("Layout Type:");
        egui::ComboBox::from_label("Layout")
            .selected_text(format!("{:?}", self.layout_type))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.layout_type, LayoutType::ForceDirected, "Force Directed");
                ui.selectable_value(&mut self.layout_type, LayoutType::Grid, "Grid");
                ui.selectable_value(&mut self.layout_type, LayoutType::Circular, "Circular");
                ui.selectable_value(&mut self.layout_type, LayoutType::Tree, "Tree");
                ui.selectable_value(&mut self.layout_type, LayoutType::Hierarchical, "Hierarchical");
            });
        
        ui.separator();
        
        // Layout-specific settings
        if matches!(self.layout_type, LayoutType::ForceDirected) {
            ui.collapsing("Force Layout Settings", |ui| {
                // This will be called from the layout module
                crate::visualization::layout::render_force_settings(ui);
            });
            ui.separator();
        }
        
        ui.add(egui::Slider::new(&mut self.zoom, 0.3..=2.5).text("Zoom"));
        
        ui.separator();
        
        ui.checkbox(&mut self.show_all_relationships, "Show File Relationships");
        ui.checkbox(&mut self.show_labels, "Show Element Labels");
        
        ui.separator();
        
        ui.label("Filter Elements:");
        ui.text_edit_singleline(&mut self.filter_text);
        
        ui.separator();
        
        if ui.button("Reset View").clicked() {
            self.zoom = 1.0;
            self.pan_offset = egui::Vec2::ZERO;
            self.selected_element = None;
            crate::visualization::layout::reset_layout();
        }
        
        if ui.button("Center View").clicked() {
            self.pan_offset = egui::Vec2::ZERO;
        }
        
        ui.separator();
        ui.label(format!("Animation: {:.0}%", self.animation_progress * 100.0));
    }
}
