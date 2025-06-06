use crate::parser::types::RelationshipType;

#[derive(Debug, Clone)]
pub struct ParseRelationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
}

impl ParseRelationship {
    pub fn new(source_id: String, target_id: String, relationship_type: RelationshipType) -> Self {
        Self {
            source_id,
            target_id,
            relationship_type,
        }
    }
}

// Relationship parsing functionality will be implemented here
