use super::types::RelationshipType;

#[derive(Debug, Clone)]
pub struct Relationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
}
