use eframe::egui;
use std::path::{Path, PathBuf};
use std::fs;

/// A simple file dialog that works without external dependencies
pub struct SimpleFileDialog {
    current_path: PathBuf,
    selected_path: Option<PathBuf>,
    entries: Vec<PathEntry>,
    error_message: Option<String>,
    needs_refresh: bool, // Add a flag to indicate refresh is needed
}

struct PathEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
}

impl Default for SimpleFileDialog {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let mut dialog = Self {
            current_path,
            selected_path: None,
            entries: Vec::new(),
            error_message: None,
            needs_refresh: true, // Initialize as true to load entries on first show
        };
        dialog.refresh_entries();
        dialog
    }
}

impl SimpleFileDialog {
    /// Show the dialog UI and return the selected path if confirmed
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        let mut result = None;
        
        // Check if we need to refresh entries from previous actions
        if self.needs_refresh {
            self.refresh_entries();
            self.needs_refresh = false;
        }
        
        // Show current path
        ui.horizontal(|ui| {
            ui.label("Current path: ");
            ui.label(self.current_path.to_string_lossy().to_string());
        });

        // Go up button
        if ui.button("⬆️ Up").clicked() {
            if let Some(parent) = self.current_path.parent() {
                self.current_path = parent.to_path_buf();
                self.needs_refresh = true; // Set flag instead of refreshing immediately
            }
        }
        
        // Common folders shortcuts - Simplified to avoid dependency issues
        ui.horizontal(|ui| {
            // Only use platform-specific paths that don't require additional dependencies
            if cfg!(windows) {
                for drive in ['C', 'D', 'E', 'F'] {
                    let drive_path = format!("{}:\\", drive);
                    if Path::new(&drive_path).exists() {
                        if ui.button(format!("{}:", drive)).clicked() {
                            self.current_path = PathBuf::from(drive_path);
                            self.needs_refresh = true; // Set flag instead of refreshing immediately
                        }
                    }
                }
            } else {
                // Simple home directory for non-Windows platforms
                if let Ok(home) = std::env::var("HOME") {
                    if ui.button("Home").clicked() {
                        self.current_path = PathBuf::from(&home);
                        self.needs_refresh = true; // Set flag instead of refreshing immediately
                    }
                }
            }
        });
        
        // Show error if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
        }
        
        // File/folder list
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut new_path = None;
            
            for entry in &self.entries {
                let name = if entry.is_dir {
                    format!("📁 {}", entry.name)
                } else {
                    format!("📄 {}", entry.name)
                };
                
                if ui.selectable_label(false, name).clicked() {
                    if entry.is_dir {
                        // Store the new path but don't change it yet to avoid borrowing conflicts
                        new_path = Some(entry.path.clone());
                    }
                }
            }
            
            // Apply the path change outside the loop if needed
            if let Some(path) = new_path {
                self.current_path = path;
                self.needs_refresh = true;
            }
        });
        
        // Select button
        ui.horizontal(|ui| {
            if ui.button("Select this folder").clicked() {
                self.selected_path = Some(self.current_path.clone());
            }
            
            if let Some(selected_path) = &self.selected_path {
                if ui.button("Confirm").clicked() {
                    result = Some(selected_path.to_string_lossy().to_string());
                }
            }
        });
        
        result
    }
    
    fn refresh_entries(&mut self) {
        self.entries.clear();
        self.error_message = None;
        
        match fs::read_dir(&self.current_path) {
            Ok(read_dir) => {
                let mut dirs = Vec::new();
                
                for entry_result in read_dir {
                    if let Ok(entry) = entry_result {
                        let path = entry.path();
                        let name = entry.file_name().to_string_lossy().to_string();
                        
                        // Skip hidden files/folders
                        if name.starts_with('.') {
                            continue;
                        }
                        
                        let is_dir = path.is_dir();
                        
                        dirs.push(PathEntry {
                            name,
                            path,
                            is_dir,
                        });
                    }
                }
                
                // Sort directories first, then files, both alphabetically
                dirs.sort_by(|a, b| {
                    match (a.is_dir, b.is_dir) {
                        (true, false) => std::cmp::Ordering::Less,
                        (false, true) => std::cmp::Ordering::Greater,
                        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                    }
                });
                
                self.entries = dirs;
            },
            Err(err) => {
                self.error_message = Some(format!("Error reading directory: {}", err));
            }
        }
    }
}
