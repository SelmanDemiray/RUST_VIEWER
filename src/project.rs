use std::collections::HashMap;
use std::fs;
use walkdir::WalkDir;
use crate::parser::{parse_file, CodeElement, Relationship};

#[derive(Default)]
pub struct Project {
    pub files: Vec<String>,
    pub file_contents: HashMap<String, String>,
    pub elements: Vec<CodeElement>,
    pub relationships: Vec<Relationship>,
    pub project_path: Option<String>,
}

impl Project {
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
                    self.elements.extend(elements);
                    self.relationships.extend(relationships);
                }
            }
        }
    }

    pub fn get_file_content(&self, file_path: &str) -> Option<&String> {
        self.file_contents.get(file_path)
    }
}
