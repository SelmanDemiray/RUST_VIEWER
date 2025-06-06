pub mod state;
pub mod renderer;

pub use state::*;
pub use renderer::*;

// Import layout functions from the layout module directory
pub mod layout {
    pub mod force_directed;
    
    pub use force_directed::*;
    
    pub fn reset_layout() {
        // Reset layout implementation
    }
}

// Visualization controller
pub struct Visualization {
    pub state: VisualizationState,
}

impl Visualization {
    pub fn new() -> Self {
        Self {
            state: VisualizationState::new(),
        }
    }
    
    pub fn render(&mut self, ui: &mut eframe::egui::Ui) {
        renderer::render_visualization(ui, &mut self.state);
    }
    
    pub fn update(&mut self, dt: f32) {
        self.state.update(dt);
    }
}

impl Default for Visualization {
    fn default() -> Self {
        Self::new()
    }
}
