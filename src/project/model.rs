use std::collections::HashMap;
use crate::parser::{CodeElement, Relationship};

#[derive(Default)]
pub struct Project {
    pub files: Vec<String>,
    pub file_contents: HashMap<String, String>,
    pub elements: Vec<CodeElement>,
    pub relationships: Vec<Relationship>,
    pub project_path: Option<String>,
}

impl Project {
    pub fn get_file_content(&self, file_path: &str) -> Option<&String> {
        self.file_contents.get(file_path)
    }
    
    pub fn load_project(&mut self, path: &str) {
        super::loader::load_project(self, path);
    }
}
