use eframe::egui;
use std::path::PathBuf;
use std::fs;

/// A simple file dialog implementation that doesn't depend on external crates
pub struct SimpleFileDialog {
    current_path: PathBuf,
    history: Vec<PathBuf>,
    show_hidden: bool,
    path_input: String, // Add editable path input
}

impl Default for SimpleFileDialog {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let path_str = current_dir.to_string_lossy().to_string();
        Self {
            current_path: current_dir,
            history: Vec::new(),
            show_hidden: false,
            path_input: path_str,
        }
    }
}

impl SimpleFileDialog {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut result = None;
        
        // Current path display and navigation
        ui.horizontal(|ui| {
            if ui.button("⬆ Up").clicked() {
                if let Some(parent) = self.current_path.parent() {
                    self.history.push(self.current_path.clone());
                    self.current_path = parent.to_path_buf();
                    self.path_input = self.current_path.to_string_lossy().to_string();
                }
            }
            
            if ui.button("⟲ Back").clicked() {
                if let Some(prev_path) = self.history.pop() {
                    self.current_path = prev_path;
                    self.path_input = self.current_path.to_string_lossy().to_string();
                }
            }
            
            ui.label("Path: ");
        });
        
        // Editable path input
        ui.horizontal(|ui| {
            let response = ui.text_edit_singleline(&mut self.path_input);
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                // User pressed Enter, try to navigate to the path
                let new_path = PathBuf::from(&self.path_input);
                if new_path.exists() && new_path.is_dir() {
                    self.history.push(self.current_path.clone());
                    self.current_path = new_path;
                } else {
                    // Reset to current path if invalid
                    self.path_input = self.current_path.to_string_lossy().to_string();
                }
            }
            
            if ui.button("Go").clicked() {
                let new_path = PathBuf::from(&self.path_input);
                if new_path.exists() && new_path.is_dir() {
                    self.history.push(self.current_path.clone());
                    self.current_path = new_path;
                } else {
                    // Reset to current path if invalid
                    self.path_input = self.current_path.to_string_lossy().to_string();
                }
            }
        });
        
        ui.checkbox(&mut self.show_hidden, "Show hidden files");
        
        ui.separator();
        
        // Scrollable file list
        egui::ScrollArea::vertical().max_height(300.0).show(ui, |ui| {
            if let Ok(entries) = fs::read_dir(&self.current_path) {
                let mut entries: Vec<_> = entries
                    .filter_map(Result::ok)
                    .filter(|e| {
                        if self.show_hidden {
                            true
                        } else {
                            let name = e.file_name();
                            let name_str = name.to_string_lossy();
                            !name_str.starts_with(".")
                        }
                    })
                    .collect();
                
                // Sort entries: directories first, then files, both alphabetically
                entries.sort_by(|a, b| {
                    let a_is_dir = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                    let b_is_dir = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                    
                    match (a_is_dir, b_is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.file_name().cmp(&b.file_name()),
                    }
                });
                
                for entry in entries {
                    let path = entry.path();
                    let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                    let name = entry.file_name();
                    let name_str = name.to_string_lossy();
                    
                    let label_text = if is_dir {
                        format!("📁 {}/", name_str)
                    } else {
                        format!("📄 {}", name_str)
                    };
                    
                    if ui.selectable_label(false, label_text).clicked() {
                        if is_dir {
                            self.history.push(self.current_path.clone());
                            self.current_path = path;
                            self.path_input = self.current_path.to_string_lossy().to_string();
                        }
                        // For files, we don't navigate since this is a folder picker
                    }
                }
            } else {
                ui.label("Error reading directory");
            }
        });
        
        ui.separator();
        
        // Current selection display
        ui.horizontal(|ui| {
            ui.label("Selected folder:");
            ui.label(self.current_path.to_string_lossy().to_string());
        });
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Select This Folder").clicked() {
                result = Some(self.current_path.to_string_lossy().to_string());
            }
            
            if ui.button("Cancel").clicked() {
                // Return empty result to indicate cancellation
                result = Some(String::new());
            }
        });
        
        result
    }
}
