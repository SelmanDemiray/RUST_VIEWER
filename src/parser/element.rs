use crate::parser::types::{CodeElement, ElementType};

impl CodeElement {
    pub fn new(id: String, name: String, element_type: ElementType, file_path: String) -> Self {
        Self {
            id,
            name,
            element_type,
            file_path,
            start_line: 0,
            end_line: 0,
        }
    }
}

// Element parsing functionality will be implemented here
