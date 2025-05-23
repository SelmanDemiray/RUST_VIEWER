use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use crate::parser::{parse_file, ElementType, RelationshipType};

#[derive(Default)]
pub struct Project {
    pub files: Vec<String>,
    pub elements: Vec<Element>,
    pub relationships: Vec<Relationship>,
    pub project_path: Option<String>,
    pub file_contents: HashMap<String, String>,
}

impl Project {
    pub fn get_file_content(&self, file_path: &str) -> Option<&str> {
        self.file_contents.get(file_path).map(|s| s.as_str())
    }
    
    pub fn load_project(&mut self, path: &str) {
        self.project_path = Some(path.to_string());
        self.files.clear();
        self.file_contents.clear();
        self.elements.clear();
        self.relationships.clear();

        // Walk through the directory and find Rust files
        for entry in WalkDir::new(path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
        {
            let file_path = entry.path().to_str().unwrap().to_string();
            let relative_path = file_path.strip_prefix(path).unwrap_or(&file_path).to_string();
            let normalized_path = relative_path.replace('\\', "/");
            
            self.files.push(normalized_path.clone());
            
            // Read file content
            if let Ok(content) = fs::read_to_string(&file_path) {
                self.file_contents.insert(normalized_path.clone(), content.clone());
                
                // Parse the file to extract code elements and relationships
                if let Ok((elements, relationships)) = parse_file(&normalized_path, &content) {
                    // Convert parser::CodeElement to project::Element
                    for element in elements {
                        self.elements.push(Element {
                            id: element.id,
                            name: element.name,
                            file_path: element.file_path,
                            element_type: element.element_type,
                        });
                    }
                    
                    // Convert parser::Relationship to project::Relationship
                    for rel in relationships {
                        self.relationships.push(Relationship {
                            source_id: rel.source_id,
                            target_id: rel.target_id,
                            relationship_type: rel.relationship_type,
                        });
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct Element {
    pub id: String,
    pub name: String,
    pub file_path: String,
    #[allow(dead_code)]
    pub element_type: ElementType,
}

#[derive(Debug, Clone)]
pub struct Relationship {
    pub source_id: String,
    pub target_id: String,
    pub relationship_type: RelationshipType,
}
