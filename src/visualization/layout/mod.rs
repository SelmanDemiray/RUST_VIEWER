mod force_directed;
mod grid;
mod circular;
mod tree;
mod hierarchical;

use eframe::egui;
use crate::project::Project;
use crate::visualization::state::LayoutType;
use std::collections::HashMap;

pub use force_directed::ForceDirectedLayout;

#[derive(Debug, Clone)]
#[allow(dead_code)] // These structs are part of the layout system for future development
pub struct LayoutNode {
    pub id: String,
    pub label: String,
    pub size: f32,
}

#[derive(Debug, Clone)]
#[allow(dead_code)] // These structs are part of the layout system for future development
pub struct LayoutEdge {
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Edge types for future layout implementations
pub enum EdgeType {
    Contains,
    Calls,
    Implements,
}

pub fn calculate_positions(
    project: &Project,
    layout_type: &LayoutType,
    zoom: f32,
    center: egui::Pos2,
    animation_progress: f32,
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    match layout_type {
        LayoutType::ForceDirected => {
            let mut file_positions = HashMap::new();
            let mut element_positions = HashMap::new();
            let layout = ForceDirectedLayout::default();
            layout.calculate_positions(project, &mut file_positions, &mut element_positions, zoom, center);
            (file_positions, element_positions)
        },
        LayoutType::Grid => {
            grid::calculate(project, zoom, center, animation_progress)
        },
        LayoutType::Circular => {
            circular::calculate(project, zoom, center, animation_progress)
        },
        LayoutType::Tree => {
            tree::calculate(project, zoom, center, animation_progress)
        },
        LayoutType::Hierarchical => {
            hierarchical::calculate(project, zoom, center, animation_progress)
        },
    }
}
