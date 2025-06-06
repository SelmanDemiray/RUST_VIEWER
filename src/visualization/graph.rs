use eframe::egui;
use crate::parser::types::{CodeElement, RelationshipType};

pub struct GraphVisualization {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub position: egui::Pos2,
    pub size: egui::Vec2,
    pub element: CodeElement,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relationship_type: RelationshipType,
}

impl GraphVisualization {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }
    
    pub fn render(&self, ui: &mut egui::Ui) {
        // Basic graph rendering
        for node in &self.nodes {
            let rect = egui::Rect::from_min_size(node.position, node.size);
            ui.allocate_rect(rect, egui::Sense::click());
            ui.painter().rect_filled(rect, 5.0, egui::Color32::BLUE);
        }
    }
}
