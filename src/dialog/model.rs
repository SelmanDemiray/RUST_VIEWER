use std::path::{Path, PathBuf};
use std::collections::VecDeque;
use eframe::egui;

use super::{
    view_modes::ViewMode,
    renderer,
};

pub struct PathEntry {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: Option<u64>,
    pub last_modified: Option<String>,
}

/// A simple file dialog that works without external dependencies
pub struct SimpleFileDialog {
    pub current_path: PathBuf,
    pub selected_path: Option<PathBuf>,
    pub entries: Vec<PathEntry>,
    pub error_message: Option<String>,
    pub needs_refresh: bool,
    pub bookmarks: Vec<PathBuf>,
    pub recent_paths: VecDeque<PathBuf>,
    pub search_query: String,
    pub view_mode: ViewMode,
    pub breadcrumbs: Vec<PathBuf>,
    pub max_recent_paths: usize, // New field for configurable limit
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
            
            // Scan for all available drives on Windows (A-Z)
            for drive_letter in 'A'..='Z' {
                let drive_path = format!("{}:\\", drive_letter);
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
            max_recent_paths: 10, // Default to 10 recent paths
        };
        
        dialog.refresh_entries();
        dialog.update_breadcrumbs();
        
        dialog
    }
}

impl SimpleFileDialog {
    /// Show the dialog UI and return the selected path if confirmed
    pub fn show(&mut self, ui: &mut egui::Ui) -> Option<String> {
        if self.needs_refresh {
            self.refresh_entries();
            self.update_breadcrumbs();
            self.needs_refresh = false;
        }
        
        renderer::render(self, ui)
    }
    
    pub fn refresh_entries(&mut self) {
        super::utils::refresh_entries(self);
    }
    
    pub fn update_breadcrumbs(&mut self) {
        super::utils::update_breadcrumbs(self);
    }
    
    pub fn add_to_recent_paths(&mut self, path: PathBuf) {
        // Remove if already exists to avoid duplicates
        self.recent_paths.retain(|p| p != &path);
        
        // Add to front
        self.recent_paths.push_front(path);
        
        // Keep only the configured number of entries
        while self.recent_paths.len() > self.max_recent_paths {
            self.recent_paths.pop_back();
        }
    }
    
    /// Get a reference to the recent paths
    pub fn get_recent_paths(&self) -> &VecDeque<PathBuf> {
        &self.recent_paths
    }
    
    /// Clear all recent paths
    pub fn clear_recent_paths(&mut self) {
        self.recent_paths.clear();
    }
    
    /// Set the maximum number of recent paths to store
    pub fn set_max_recent_paths(&mut self, max: usize) {
        self.max_recent_paths = max;
        
        // Trim the list if needed after changing the limit
        while self.recent_paths.len() > self.max_recent_paths {
            self.recent_paths.pop_back();
        }
    }
}
