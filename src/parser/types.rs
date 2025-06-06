#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    Function,
    Struct,
    Enum,
    Trait,
    Module,
    Impl,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    Imports,
    Uses,
    Extends,
    Implements,
    Contains,
    Calls,
    Instantiates,
    References,
    DependsOn,
    AssociatedWith,
}

#[derive(Debug, Clone)]
pub struct CodeElement {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
}
