use std::collections::HashMap;
use std::path::PathBuf;
use crate::project::{Element, Relationship};

#[derive(Debug, Clone, Default)]
pub struct ProjectModel {
    pub name: String,
    pub root_path: String,
    pub project_path: PathBuf,
    pub files: Vec<String>,
    pub file_contents: HashMap<String, String>,
    pub elements: Vec<Element>,
    pub relationships: Vec<Relationship>,
}

impl ProjectModel {
    pub fn new(name: String, root_path: String) -> Self {
        Self {
            name,
            project_path: PathBuf::from(root_path.clone()),
            root_path,
            files: Vec::new(),
            file_contents: HashMap::new(),
            elements: Vec::new(),
            relationships: Vec::new(),
        }
    }

    pub fn load_from_path(path: &str) -> Result<Self, String> {
        let mut project = Self::new("".to_string(), path.to_string());
        project.project_path = PathBuf::from(path);
        
        // Use walkdir for recursive scanning
        for entry in walkdir::WalkDir::new(path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    if extension == "rs" {
                        if let Some(path_str) = file_path.to_str() {
                            // Store relative path
                            let relative_path = file_path
                                .strip_prefix(path)
                                .unwrap_or(file_path)
                                .to_string_lossy()
                                .replace('\\', "/");
                            
                            project.files.push(relative_path.clone());
                            
                            // Try to read file content
                            if let Ok(content) = std::fs::read_to_string(&file_path) {
                                project.file_contents.insert(relative_path, content);
                            }
                        }
                    }
                }
            }
        }
        
        Ok(project)
    }

    pub fn name(&self) -> String {
        self.project_path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("Unknown Project")
            .to_string()
    }

    pub fn path(&self) -> String {
        self.project_path.to_string_lossy().to_string()
    }
}
