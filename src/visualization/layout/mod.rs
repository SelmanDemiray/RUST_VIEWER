pub mod force_directed;
// We'll comment out the missing modules until they're implemented
// mod circular;
// mod grid;
mod tree;
// mod hierarchical;

use crate::project::Project;
use crate::visualization::state::LayoutType;
use eframe::egui;
use std::collections::HashMap;

pub fn calculate_positions(
    project: &Project,
    layout_type: &LayoutType,
    zoom: f32,
    center: egui::Pos2,
    animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    // Delegate to the appropriate layout module based on layout_type
    match layout_type {
        // For now, use force_directed for all layouts
        // since we haven't implemented the other layouts yet
        LayoutType::Grid => force_directed::calculate(project, zoom, center, animation_progress),
        LayoutType::Circular => force_directed::calculate(project, zoom, center, animation_progress),
        LayoutType::Tree => tree::calculate(project, zoom, center, animation_progress),
        LayoutType::ForceDirected => force_directed::calculate(project, zoom, center, animation_progress),
        LayoutType::Hierarchical => force_directed::calculate(project, zoom, center, animation_progress),
    }
}
