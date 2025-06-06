// Auto-generated mod declarations
pub mod loader;
pub mod model;

// Use the ProjectModel from the model module
pub use model::ProjectModel;

// Create a type alias instead of conflicting struct
pub type Project = ProjectModel;

#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub element_type: crate::parser::ElementType,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: crate::parser::RelationshipType,
}
