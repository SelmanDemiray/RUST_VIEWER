use crate::project::ProjectModel;
use eframe::egui;

pub mod text_editor;
pub mod renderer;
pub mod code_editor;

pub use code_editor::CodeEditor;

pub struct Editor {
    pub code_editor: CodeEditor,
    pub current_file: Option<String>,
    pub current_content: String,
    pub is_modified: bool,
    pub expanded_folders: std::collections::HashSet<String>,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            code_editor: CodeEditor::new(),
            current_file: None,
            current_content: String::new(),
            is_modified: false,
            expanded_folders: std::collections::HashSet::new(),
        }
    }
}

impl Editor {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn render(&mut self, ui: &mut egui::Ui, project: &ProjectModel) {
        ui.horizontal(|ui| {
            // File list on the left
            ui.vertical(|ui| {
                ui.set_width(250.0);
                ui.heading("Project Files");
                ui.separator();
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.render_file_tree(ui, project);
                });
            });
            
            ui.separator();
            
            // Editor on the right
            ui.vertical(|ui| {
                // Clone current_file to avoid borrowing issues
                if let Some(current_file) = self.current_file.clone() {
                    self.render_editor_content(ui, &current_file);
                } else {
                    ui.vertical_centered(|ui| {
                        ui.heading("Code Editor");
                        ui.add_space(50.0);
                        ui.label("Select a file from the project tree to start editing");
                        ui.add_space(20.0);
                        ui.label("💡 Tip: Click on any .rs file in the tree to open it");
                    });
                }
            });
        });
    }
    
    fn render_file_tree(&mut self, ui: &mut egui::Ui, project: &ProjectModel) {
        // Group files by directory
        let mut file_tree = std::collections::BTreeMap::new();
        
        for file in &project.files {
            let parts: Vec<&str> = file.split('/').collect();
            let mut current_path = Vec::new();
            
            for (i, part) in parts.iter().enumerate() {
                current_path.push(*part);
                
                if i == parts.len() - 1 {
                    // This is a file - insert into the tree structure
                    let mut current_map = &mut file_tree;
                    
                    // Navigate to the correct directory level
                    for dir_part in &current_path[..current_path.len() - 1] {
                        let entry = current_map.entry(dir_part.to_string())
                            .or_insert_with(|| FileTreeNode::Directory(std::collections::BTreeMap::new()));
                        
                        if let FileTreeNode::Directory(ref mut dir_map) = entry {
                            current_map = dir_map;
                        }
                    }
                    
                    // Insert the file
                    current_map.insert(part.to_string(), FileTreeNode::File(file.clone()));
                }
            }
        }
        
        self.render_tree_node(ui, "", &file_tree, project);
    }
    
    fn render_tree_node(&mut self, ui: &mut egui::Ui, path_prefix: &str, tree: &std::collections::BTreeMap<String, FileTreeNode>, project: &ProjectModel) {
        for (name, node) in tree {
            let full_path = if path_prefix.is_empty() {
                name.clone()
            } else {
                format!("{}/{}", path_prefix, name)
            };
            
            match node {
                FileTreeNode::File(file_path) => {
                    let selected = self.current_file.as_ref() == Some(file_path);
                    let icon = if file_path.ends_with(".rs") { "🦀" } else { "📄" };
                    
                    if ui.selectable_label(selected, format!("{} {}", icon, name)).clicked() {
                        self.open_file(file_path, project);
                    }
                }
                FileTreeNode::Directory(children) => {
                    let expanded = self.expanded_folders.contains(&full_path);
                    
                    let response = ui.horizontal(|ui| {
                        let icon = if expanded { "📂" } else { "📁" };
                        ui.selectable_label(false, format!("{} {}", icon, name))
                    }).inner;
                    
                    if response.clicked() {
                        if expanded {
                            self.expanded_folders.remove(&full_path);
                        } else {
                            self.expanded_folders.insert(full_path.clone());
                        }
                    }
                    
                    if expanded {
                        ui.indent(format!("indent_{}", full_path), |ui| {
                            self.render_tree_node(ui, &full_path, children, project);
                        });
                    }
                }
            }
        }
    }
    
    fn render_editor_content(&mut self, ui: &mut egui::Ui, current_file: &str) {
        // File header with save/reload buttons
        ui.horizontal(|ui| {
            ui.heading("Editor");
            ui.separator();
            ui.label(current_file);
            if self.is_modified {
                ui.label(egui::RichText::new("● Modified").color(egui::Color32::YELLOW));
            }
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("💾 Save").clicked() {
                    self.save_current_file();
                }
                
                if ui.button("🔄 Reload").clicked() {
                    // Reload would go here
                    self.is_modified = false;
                }
            });
        });
        
        ui.separator();
        
        // Editor content
        let previous_content = self.current_content.clone();
        let response = ui.add(
            egui::TextEdit::multiline(&mut self.current_content)
                .desired_width(f32::INFINITY)
                .desired_rows(30)
                .code_editor()
        );
        
        // Check if content was modified
        if self.current_content != previous_content {
            self.is_modified = true;
        }
        
        // Show cursor position and file stats
        if response.has_focus() {
            ui.horizontal(|ui| {
                ui.label(format!("Lines: {}", self.current_content.lines().count()));
                ui.separator();
                ui.label(format!("Characters: {}", self.current_content.len()));
            });
        }
    }
    
    pub fn open_file(&mut self, file_path: &str, project: &ProjectModel) {
        self.current_file = Some(file_path.to_string());
        
        if let Some(content) = project.file_contents.get(file_path) {
            self.current_content = content.clone();
        } else {
            self.current_content = String::new();
        }
        
        self.is_modified = false;
    }
    
    fn save_current_file(&mut self) {
        if let Some(ref file_path) = self.current_file {
            // In a real implementation, you would save to the actual file
            println!("Saving file: {}", file_path);
            self.is_modified = false;
        }
    }
}

#[derive(Debug)]
enum FileTreeNode {
    File(String),
    Directory(std::collections::BTreeMap<String, FileTreeNode>),
}

// Helper function to render a file tree with proper project structure
pub fn render_project_tree(
    ui: &mut egui::Ui,
    project: &ProjectModel,
    current_file: &Option<String>,
    selected_file: &mut Option<String>,
    expanded_folders: &mut std::collections::HashSet<String>,
) {
    ui.heading("Project Structure");
    ui.separator();
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        // Group files by directory for tree view
        let mut dirs: std::collections::BTreeMap<String, Vec<String>> = std::collections::BTreeMap::new();
        
        for file in &project.files {
            if let Some(dir) = file.rfind('/') {
                let directory = &file[..dir];
                let filename = &file[dir + 1..];
                dirs.entry(directory.to_string()).or_default().push(filename.to_string());
            } else {
                dirs.entry("".to_string()).or_default().push(file.clone());
            }
        }
        
        for (dir, files) in dirs {
            if !dir.is_empty() {
                let expanded = expanded_folders.contains(&dir);
                let folder_icon = if expanded { "📂" } else { "📁" };
                
                if ui.selectable_label(false, format!("{} {}", folder_icon, dir)).clicked() {
                    if expanded {
                        expanded_folders.remove(&dir);
                    } else {
                        expanded_folders.insert(dir.clone());
                    }
                }
                
                if expanded {
                    ui.indent(format!("folder_{}", dir), |ui| {
                        for file in files {
                            let full_path = format!("{}/{}", dir, file);
                            let is_selected = current_file.as_ref() == Some(&full_path);
                            
                            if ui.selectable_label(is_selected, format!("🦀 {}", file)).clicked() {
                                *selected_file = Some(full_path);
                            }
                        }
                    });
                }
            } else {
                // Root level files
                for file in files {
                    let is_selected = current_file.as_ref() == Some(&file);
                    if ui.selectable_label(is_selected, format!("🦀 {}", file)).clicked() {
                        *selected_file = Some(file);
                    }
                }
            }
        }
    });
}
