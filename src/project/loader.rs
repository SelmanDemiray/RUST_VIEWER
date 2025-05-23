use std::fs;
use walkdir::WalkDir;
use super::model::Project;
use crate::parser;

pub fn load_project(project: &mut Project, path: &str) {
    project.project_path = Some(path.to_string());
    project.files.clear();
    project.file_contents.clear();
    project.elements.clear();
    project.relationships.clear();

    // Walk through the directory and find Rust files
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let file_path = entry.path().to_str().unwrap().to_string();
        let relative_path = file_path.strip_prefix(path).unwrap_or(&file_path).to_string();
        let normalized_path = relative_path.replace('\\', "/");
        
        project.files.push(normalized_path.clone());
        
        // Read file content
        if let Ok(content) = fs::read_to_string(&file_path) {
            project.file_contents.insert(normalized_path.clone(), content.clone());
            
            // Parse the file to extract code elements and relationships
            if let Ok((elements, relationships)) = parser::analyze_file(&normalized_path, &content) {
                project.elements.extend(elements);
                project.relationships.extend(relationships);
            }
        }
    }
}
