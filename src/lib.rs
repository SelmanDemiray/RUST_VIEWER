pub mod project;
pub mod parser;
pub mod dialog;
pub mod simple_dialog;
pub mod visualization;
pub mod ui;
pub mod app;
pub mod editor;

// Re-export commonly used types
pub use project::Project;
pub use visualization::VisualizationState;
pub use app::App;

#[derive(Debug, Clone)]
pub struct LayoutSettings {
    pub force_strength: f32,
    pub repulsion_strength: f32,
    pub spring_length: f32,
    pub damping: f32,
    pub show_dependencies: bool,
    pub show_functions: bool,
    pub show_structs: bool,
}

impl Default for LayoutSettings {
    fn default() -> Self {
        Self {
            force_strength: 0.1,
            repulsion_strength: 100.0,
            spring_length: 50.0,
            damping: 0.9,
            show_dependencies: true,
            show_functions: true,
            show_structs: true,
        }
    }
}
