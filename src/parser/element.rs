use super::types::ElementType;

#[derive(Debug, Clone)]
#[allow(dead_code)]  // Allow unused fields for future development
pub struct CodeElement {
    pub id: String,
    pub name: String,
    pub element_type: ElementType,
    pub file_path: String,
    pub start_line: usize,
    pub end_line: usize,
}
