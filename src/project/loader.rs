use crate::project::model::{ProjectModel, Element, Relationship, ElementType};
use std::path::Path;
use std::fs;

pub struct ProjectLoader;

impl ProjectLoader {
    pub fn new() -> Self {
        Self
    }

    pub fn load_project(&mut self, project_path: &str) -> Result<ProjectModel, String> {
        let path = Path::new(project_path);
        
        if !path.exists() {
            return Err(format!("Path does not exist: {}", project_path));
        }

        let mut project = ProjectModel::new(project_path.to_string());
        
        // Find all Rust files
        self.scan_directory(&mut project, path)?;
        
        // Parse each file and extract elements
        for file_path in project.files.clone() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                project.file_contents.insert(file_path.clone(), content.clone());
                self.parse_rust_file(&mut project, &content, &file_path);
            }
        }

        // Find relationships between elements
        self.find_relationships(&mut project);

        Ok(project)
    }

    fn scan_directory(&self, project: &mut ProjectModel, dir: &Path) -> Result<(), String> {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        // Skip target and other build directories
                        if let Some(dir_name) = path.file_name() {
                            if dir_name == "target" || dir_name == ".git" {
                                continue;
                            }
                        }
                        self.scan_directory(project, &path)?;
                    } else if let Some(extension) = path.extension() {
                        if extension == "rs" {
                            if let Some(path_str) = path.to_str() {
                                project.files.push(path_str.to_string());
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_rust_file(&mut self, project: &mut ProjectModel, content: &str, file_path: &str) {
        // Simple parsing - look for function definitions, structs, etc.
        let lines: Vec<&str> = content.lines().collect();
        
        for (line_num, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            
            // Parse function definitions
            if let Some(func_name) = self.extract_function_name(trimmed) {
                let element = Element {
                    id: format!("{}::{}", file_path, func_name),
                    name: func_name,
                    file_path: file_path.to_string(),
                    element_type: ElementType::Function,
                };
                project.elements.push(element);
            }
            
            // Parse struct definitions
            if let Some(struct_name) = self.extract_struct_name(trimmed) {
                let element = Element {
                    id: format!("{}::{}", file_path, struct_name),
                    name: struct_name,
                    file_path: file_path.to_string(),
                    element_type: ElementType::Struct,
                };
                project.elements.push(element);
            }
            
            // Parse enum definitions
            if let Some(enum_name) = self.extract_enum_name(trimmed) {
                let element = Element {
                    id: format!("{}::{}", file_path, enum_name),
                    name: enum_name,
                    file_path: file_path.to_string(),
                    element_type: ElementType::Enum,
                };
                project.elements.push(element);
            }
        }
    }

    fn extract_function_name(&self, line: &str) -> Option<String> {
        if line.starts_with("fn ") || line.contains(" fn ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "fn" && i + 1 < parts.len() {
                    let name_part = parts[i + 1];
                    if let Some(paren_pos) = name_part.find('(') {
                        return Some(name_part[..paren_pos].to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_struct_name(&self, line: &str) -> Option<String> {
        if line.starts_with("struct ") || line.contains(" struct ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "struct" && i + 1 < parts.len() {
                    let name_part = parts[i + 1];
                    // Remove generic parameters if present
                    if let Some(bracket_pos) = name_part.find('<') {
                        return Some(name_part[..bracket_pos].to_string());
                    } else if let Some(brace_pos) = name_part.find('{') {
                        return Some(name_part[..brace_pos].to_string());
                    } else {
                        return Some(name_part.to_string());
                    }
                }
            }
        }
        None
    }

    fn extract_enum_name(&self, line: &str) -> Option<String> {
        if line.starts_with("enum ") || line.contains(" enum ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            for (i, part) in parts.iter().enumerate() {
                if *part == "enum" && i + 1 < parts.len() {
                    let name_part = parts[i + 1];
                    if let Some(brace_pos) = name_part.find('{') {
                        return Some(name_part[..brace_pos].to_string());
                    } else {
                        return Some(name_part.to_string());
                    }
                }
            }
        }
        None
    }

    fn find_relationships(&mut self, project: &mut ProjectModel) {
        let elements = project.elements.clone();
        
        for element in &elements {
            if let Some(content) = project.file_contents.get(&element.file_path) {
                self.find_local_dependencies(project, element, content);
                self.find_cross_file_dependencies(project, element, content);
            }
        }
    }

    fn find_local_dependencies(&mut self, project: &mut ProjectModel, element: &Element, content: &str) {
        for other_element in &project.elements.clone() {
            if element.id != other_element.id && content.contains(&other_element.name) {
                let relationship = Relationship {
                    from_id: element.id.clone(),
                    to_id: other_element.id.clone(),
                    relationship_type: "calls".to_string(),
                };
                project.relationships.push(relationship);
            }
        }
    }

    fn find_cross_file_dependencies(&mut self, project: &mut ProjectModel, element: &Element, content: &str) {
        let lines: Vec<&str> = content.lines().collect();
        
        for line in lines {
            if line.trim().starts_with("use ") {
                if let Some(target_element) = self.find_element_by_use_statement(project, line.trim()) {
                    let relationship = Relationship {
                        from_id: element.id.clone(),
                        to_id: target_element.id.clone(),
                        relationship_type: "imports".to_string(),
                    };
                    project.relationships.push(relationship);
                }
            }
        }
    }

    fn find_element_by_use_statement<'a>(&self, project: &'a ProjectModel, use_statement: &str) -> Option<&'a Element> {
        // Simple use statement parsing
        if let Some(import_path) = use_statement.strip_prefix("use ") {
            let import_path = import_path.trim_end_matches(';').trim();
            
            for element in &project.elements {
                if import_path.contains(&element.name) {
                    return Some(element);
                }
            }
        }
        None
    }
}

// Public function for loading projects
pub fn load_project_from_path(path: &str) -> Result<ProjectModel, String> {
    let mut loader = ProjectLoader::new();
    loader.load_project(path)
}
