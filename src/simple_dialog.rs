use eframe::egui;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::VecDeque;

/// A simple file dialog that works without external dependencies
pub struct SimpleFileDialog {
    current_path: PathBuf,
    selected_path: Option<PathBuf>,
    entries: Vec<PathEntry>,
    error_message: Option<String>,
    needs_refresh: bool,
    bookmarks: Vec<PathBuf>,
    recent_paths: VecDeque<PathBuf>,
    search_query: String,
    view_mode: ViewMode,
    breadcrumbs: Vec<PathBuf>,
}

#[derive(PartialEq)]
enum ViewMode {
    List,
    Grid,
}

struct PathEntry {
    name: String,
    path: PathBuf,
    is_dir: bool,
    size: Option<u64>,       // File size in bytes
    last_modified: Option<String>,  // Last modified time
}

impl Default for SimpleFileDialog {
    fn default() -> Self {
        let current_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        // Set up default bookmarks
        let mut bookmarks = Vec::new();
        
        // Add system-specific default bookmarks
        if cfg!(windows) {
            if let Ok(home) = std::env::var("USERPROFILE") {
                bookmarks.push(PathBuf::from(&home));
                bookmarks.push(PathBuf::from(format!("{}/Desktop", home)));
                bookmarks.push(PathBuf::from(format!("{}/Documents", home)));
            }
            
            for drive in ['C', 'D', 'E', 'F'] {
                let drive_path = format!("{}:\\", drive);
                if Path::new(&drive_path).exists() {
                    bookmarks.push(PathBuf::from(drive_path));
                }
            }
        } else {
            // Unix-like systems
            if let Ok(home) = std::env::var("HOME") {
                bookmarks.push(PathBuf::from(&home));
                bookmarks.push(PathBuf::from(format!("{}/Documents", home)));
                bookmarks.push(PathBuf::from(format!("{}/Desktop", home)));
            }
            bookmarks.push(PathBuf::from("/"));
        }
        
        let mut dialog = Self {
            current_path,
            selected_path: None,
            entries: Vec::new(),
            error_message: None,
            needs_refresh: true,
            bookmarks,
            recent_paths: VecDeque::with_capacity(10),
            search_query: String::new(),
            view_mode: ViewMode::List,
            breadcrumbs: Vec::new(),
        };
        
        dialog.refresh_entries();
        dialog.update_breadcrumbs();
        
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
            self.update_breadcrumbs();
            self.needs_refresh = false;
        }
        
        // Dialog header with navigation and view controls
        self.show_header(ui);
        
        // Show error if any
        if let Some(error) = &self.error_message {
            ui.colored_label(egui::Color32::RED, error);
            if ui.button("Clear Error").clicked() {
                self.error_message = None;
            }
        }
        
        // Left sidebar for bookmarks and recent folders
        ui.columns(2, |columns| {
            // Left column: bookmarks and recent folders
            columns[0].vertical_centered(|ui| {
                ui.set_width(150.0);
                self.show_sidebar(ui);
            });
            
            // Right column: file browser
            columns[1].vertical(|ui| {
                // Breadcrumbs navigation bar
                self.show_breadcrumbs(ui);
                
                // Search box
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    if ui.text_edit_singleline(&mut self.search_query).changed() {
                        self.refresh_entries();
                    }
                    if ui.button("Clear").clicked() {
                        self.search_query.clear();
                        self.refresh_entries();
                    }
                });
                
                // File/folder list with scroll area
                self.show_file_list(ui);
            });
        });
        
        // Bottom area for selection and buttons
        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Selected path: ");
            
            let current_path_string = self.current_path.to_string_lossy().to_string();
            ui.text_edit_singleline(&mut current_path_string.to_owned())
                .on_hover_text("Current directory path");
                
            if ui.button("Select this folder").clicked() {
                self.selected_path = Some(self.current_path.clone());
                self.add_to_recent_paths(self.current_path.clone());
            }
            
            if let Some(selected_path) = &self.selected_path {
                ui.label(format!("Selected: {}", selected_path.to_string_lossy()));
                if ui.button("✓ Confirm").clicked() {
                    result = Some(selected_path.to_string_lossy().to_string());
                }
            }
        });
        
        result
    }
    
    fn show_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            // Navigation buttons
            if ui.button("⬆️ Parent").clicked() {
                if let Some(parent) = self.current_path.parent() {
                    self.current_path = parent.to_path_buf();
                    self.needs_refresh = true;
                }
            }
            
            if ui.button("🔄 Refresh").clicked() {
                self.needs_refresh = true;
            }
            
            ui.separator();
            
            // View mode toggle
            if ui.selectable_label(self.view_mode == ViewMode::List, "📋 List").clicked() {
                self.view_mode = ViewMode::List;
            }
            
            if ui.selectable_label(self.view_mode == ViewMode::Grid, "📊 Grid").clicked() {
                self.view_mode = ViewMode::Grid;
            }
            
            ui.separator();
            
            // Home button
            if ui.button("🏠 Home").clicked() {
                if let Ok(home) = if cfg!(windows) { std::env::var("USERPROFILE") } else { std::env::var("HOME") } {
                    self.current_path = PathBuf::from(home);
                    self.needs_refresh = true;
                }
            }
        });
    }
    
    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        // Bookmarks section
        ui.collapsing("📚 Bookmarks", |ui| {
            for bookmark in &self.bookmarks {
                let name = bookmark.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| {
                        bookmark.to_string_lossy().to_string()
                    });
                
                if ui.button(&name).clicked() {
                    if bookmark.exists() {
                        self.current_path = bookmark.clone();
                        self.needs_refresh = true;
                    } else {
                        self.error_message = Some(format!("Bookmark path doesn't exist: {}", bookmark.to_string_lossy()));
                    }
                }
            }
            
            // Add current path as bookmark
            if ui.button("➕ Add current").clicked() {
                if !self.bookmarks.contains(&self.current_path) {
                    self.bookmarks.push(self.current_path.clone());
                }
            }
        });
        
        // Recent paths
        ui.collapsing("🕒 Recent", |ui| {
            for recent in &self.recent_paths {
                let name = recent.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| recent.to_string_lossy().to_string());
                
                if ui.button(&name).clicked() {
                    if recent.exists() {
                        self.current_path = recent.clone();
                        self.needs_refresh = true;
                    } else {
                        self.error_message = Some(format!("Recent path doesn't exist: {}", recent.to_string_lossy()));
                        // Could also remove this invalid path from recents
                    }
                }
            }
            
            if ui.button("🗑️ Clear history").clicked() {
                self.recent_paths.clear();
            }
        });
    }
    
    fn show_breadcrumbs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal_wrapped(|ui| {
            ui.label("📂");
            
            for (index, path) in self.breadcrumbs.iter().enumerate() {
                let name = if index == 0 {
                    if cfg!(windows) {
                        // For Windows, show drive letter for root
                        path.to_string_lossy().to_string()
                    } else {
                        // For Unix, show / for root
                        "/".to_string()
                    }
                } else {
                    path.file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "?".to_string())
                };
                
                if ui.link(&name).clicked() {
                    // Recreate full path from breadcrumb
                    if cfg!(windows) {
                        // For Windows paths
                        let mut new_path = PathBuf::from(&self.breadcrumbs[0]);
                        for i in 1..=index {
                            new_path.push(self.breadcrumbs[i].file_name().unwrap_or_default());
                        }
                        self.current_path = new_path;
                    } else {
                        // For Unix paths
                        if index == 0 {
                            self.current_path = PathBuf::from("/");
                        } else {
                            let mut new_path = PathBuf::from("/");
                            for i in 1..=index {
                                new_path.push(self.breadcrumbs[i].file_name().unwrap_or_default());
                            }
                            self.current_path = new_path;
                        }
                    }
                    
                    self.needs_refresh = true;
                }
                
                if index < self.breadcrumbs.len() - 1 {
                    ui.label("›");
                }
            }
        });
    }
    
    fn show_file_list(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut new_path = None;
            
            match self.view_mode {
                ViewMode::List => {
                    // List view
                    for entry in &self.entries {
                        ui.horizontal(|ui| {
                            let icon = if entry.is_dir { "📁" } else { "📄" };
                            ui.label(icon);
                            
                            if ui.selectable_label(false, &entry.name)
                                .on_hover_ui(|ui| {
                                    // Show details on hover
                                    ui.label(format!("Path: {}", entry.path.to_string_lossy()));
                                    if let Some(size) = entry.size {
                                        ui.label(format!("Size: {} bytes", size));
                                    }
                                    if let Some(modified) = &entry.last_modified {
                                        ui.label(format!("Modified: {}", modified));
                                    }
                                })
                                .clicked() 
                            {
                                if entry.is_dir {
                                    new_path = Some(entry.path.clone());
                                }
                            }
                            
                            if !entry.is_dir {
                                if let Some(size) = entry.size {
                                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                        ui.label(format_file_size(size));
                                    });
                                }
                            }
                        });
                    }
                },
                ViewMode::Grid => {
                    // Grid view
                    egui::Grid::new("file_grid")
                        .num_columns(4)
                        .spacing([20.0, 10.0])
                        .striped(true)
                        .show(ui, |ui| {
                            let mut count = 0;
                            for entry in &self.entries {
                                let icon = if entry.is_dir { "📁" } else { "📄" };
                                
                                ui.vertical_centered(|ui| {
                                    ui.label(icon);
                                    if ui.selectable_label(false, &entry.name)
                                        .on_hover_text(entry.path.to_string_lossy())
                                        .clicked() 
                                    {
                                        if entry.is_dir {
                                            new_path = Some(entry.path.clone());
                                        }
                                    }
                                });
                                
                                count += 1;
                                if count % 4 == 0 {
                                    ui.end_row();
                                }
                            }
                        });
                }
            }
            
            // Apply the path change outside the loop if needed
            if let Some(path) = new_path {
                self.current_path = path;
                self.needs_refresh = true;
            }
        });
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
                        
                        // Skip hidden files/folders if not on Windows
                        if !cfg!(windows) && name.starts_with('.') {
                            continue;
                        }
                        
                        // Apply search filter if any
                        if !self.search_query.is_empty() && 
                           !name.to_lowercase().contains(&self.search_query.to_lowercase()) {
                            continue;
                        }
                        
                        let is_dir = path.is_dir();
                        
                        // Get file metadata if possible
                        let (size, last_modified) = if !is_dir {
                            if let Ok(metadata) = fs::metadata(&path) {
                                let size = Some(metadata.len());
                                let last_modified = metadata.modified().ok()
                                    .map(|time| {
                                        format!("{:?}", time)
                                    });
                                (size, last_modified)
                            } else {
                                (None, None)
                            }
                        } else {
                            (None, None)
                        };
                        
                        dirs.push(PathEntry {
                            name,
                            path,
                            is_dir,
                            size,
                            last_modified,
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
    
    fn update_breadcrumbs(&mut self) {
        self.breadcrumbs.clear();
        
        // Handle different OS path styles
        let path_str = self.current_path.to_string_lossy().to_string();
        
        if cfg!(windows) {
            // Windows paths
            if let Some(root) = self.current_path.ancestors().last() {
                self.breadcrumbs.push(root.to_path_buf());
            }
            
            let parts: Vec<&str> = path_str.split('\\').collect();
            let mut current = PathBuf::new();
            
            // Add drive root (e.g., "C:")
            if !parts.is_empty() && parts[0].ends_with(':') {
                current.push(parts[0]);
                self.breadcrumbs.push(current.clone());
            }
            
            // Add each path component
            for part in parts.iter().skip(1).filter(|p| !p.is_empty()) {
                current.push(part);
                self.breadcrumbs.push(current.clone());
            }
        } else {
            // Unix paths
            self.breadcrumbs.push(PathBuf::from("/"));
            
            let parts: Vec<&str> = path_str.split('/').collect();
            let mut current = PathBuf::from("/");
            
            for part in parts.iter().skip(1).filter(|p| !p.is_empty()) {
                current.push(part);
                self.breadcrumbs.push(current.clone());
            }
        }
    }
    
    fn add_to_recent_paths(&mut self, path: PathBuf) {
        // Remove if already exists to avoid duplicates
        self.recent_paths.retain(|p| p != &path);
        
        // Add to front
        self.recent_paths.push_front(path);
        
        // Keep only the last 10 entries
        while self.recent_paths.len() > 10 {
            self.recent_paths.pop_back();
        }
    }
}

// Helper function to format file sizes in a human-readable way
fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    
    if size < KB {
        format!("{} B", size)
    } else if size < MB {
        format!("{:.1} KB", size as f64 / KB as f64)
    } else if size < GB {
        format!("{:.1} MB", size as f64 / MB as f64)
    } else {
        format!("{:.1} GB", size as f64 / GB as f64)
    }
}
