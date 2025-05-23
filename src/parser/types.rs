#[derive(Debug, Clone)]
#[allow(dead_code)]  // Allow unused variants for future development
pub enum ElementType {
    Function,
    Module,
    Struct,
    Enum,
    Trait,
    Impl,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]  // Allow unused variants for future development
pub enum RelationshipType {
    Calls,      // Function calls another function
    Imports,    // Module imports something from another module
    Implements, // Type implements a trait
    Contains,   // Parent-child relationship (e.g., module contains items)
    Extends,    // Type extends/inherits from another (for trait inheritance)
    Uses,       // Function/method uses a type as parameter or return type
    References, // Code references a type without direct usage
    DependsOn,  // General dependency between code elements
    AssociatedWith, // Items with association (like methods and their parent type)
    Instantiates,   // When code creates an instance of a type
}
