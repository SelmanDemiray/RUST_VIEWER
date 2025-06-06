use crate::parser::types::{CodeElement, RelationshipType};
use crate::project::ProjectModel;
use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum LayoutType {
    ForceDirected,
    Grid,
    Circular,
    Tree,
    Hierarchical,
}

#[derive(Debug, Clone)]
pub struct Node {
    pub id: String,
    pub element: CodeElement,
    pub position: egui::Pos2,
    pub velocity: egui::Vec2,
    pub size: egui::Vec2,
    pub color: egui::Color32,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub relationship: Relationship,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub relationship_type: RelationshipType,
    pub strength: f32,
}

impl Default for LayoutType {
    fn default() -> Self {
        LayoutType::ForceDirected
    }
}

impl Default for Relationship {
    fn default() -> Self {
        Self {
            relationship_type: RelationshipType::Contains,
            strength: 1.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct VisualizationState {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub layout_type: LayoutType,
    pub show_relationships: bool,
    pub show_functions: bool,
    pub show_structs: bool,
    pub zoom: f32,
    pub pan_offset: egui::Vec2,
    pub selected_element: Option<String>, // Add missing field
}

impl Default for VisualizationState {
    fn default() -> Self {
        Self::new()
    }
}

impl VisualizationState {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            layout_type: LayoutType::default(),
            show_relationships: true,
            show_functions: true,
            show_structs: true,
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            selected_element: None, // Initialize new field
        }
    }
    
    pub fn update_from_project(&mut self, project: &ProjectModel) {
        self.nodes.clear();
        self.edges.clear();
        
        // Convert project elements to visualization nodes
        for element in &project.elements {
            self.nodes.push(Node {
                id: element.id.clone(),
                element: crate::parser::types::CodeElement {
                    id: element.id.clone(),
                    name: element.name.clone(),
                    element_type: element.element_type.clone(),
                    file_path: element.file_path.clone(),
                    start_line: 0,
                    end_line: 0,
                },
                position: egui::Pos2::ZERO,
                velocity: egui::Vec2::ZERO,
                size: egui::Vec2::new(100.0, 50.0),
                color: match element.element_type {
                    crate::parser::types::ElementType::Function => egui::Color32::LIGHT_BLUE,
                    crate::parser::types::ElementType::Struct => egui::Color32::LIGHT_GREEN,
                    crate::parser::types::ElementType::Enum => egui::Color32::LIGHT_YELLOW,
                    _ => egui::Color32::LIGHT_GRAY,
                },
            });
        }
        
        // Convert project relationships to visualization edges
        for relationship in &project.relationships {
            self.edges.push(Edge {
                source: relationship.source_id.clone(),
                target: relationship.target_id.clone(),
                relationship: Relationship {
                    relationship_type: relationship.relationship_type.clone(),
                    strength: 1.0,
                },
            });
        }
    }
    
    pub fn update(&mut self, _dt: f32) {
        // Update visualization state (animations, layout calculations, etc.)
    }
    
    // Add the missing render method
    pub fn render(&mut self, ui: &mut egui::Ui) {
        crate::visualization::renderer::render_visualization(ui, self);
    }
}
