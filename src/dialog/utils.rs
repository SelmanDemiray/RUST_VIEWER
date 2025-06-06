use super::model::{SimpleFileDialog, PathEntry};
use std::fs;

pub fn refresh_entries(dialog: &mut SimpleFileDialog) {
    dialog.entries.clear();
    dialog.error_message = None;
    
    match fs::read_dir(&dialog.current_path) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    let name = entry.file_name().to_string_lossy().to_string();
                    let is_dir = path.is_dir();
                    
                    // Get file size and modification time
                    let size = if !is_dir {
                        entry.metadata().ok().map(|m| m.len())
                    } else {
                        None
                    };
                    
                    let last_modified = entry.metadata().ok()
                        .and_then(|m| m.modified().ok())
                        .map(|t| format!("{:?}", t));
                    
                    dialog.entries.push(PathEntry {
                        name,
                        path,
                        is_dir,
                        size,
                        last_modified,
                    });
                }
            }
            
            // Sort entries: directories first, then files
            dialog.entries.sort_by(|a, b| {
                match (a.is_dir, b.is_dir) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
        },
        Err(e) => {
            dialog.error_message = Some(format!("Error reading directory: {}", e));
        }
    }
}

pub fn update_breadcrumbs(dialog: &mut SimpleFileDialog) {
    dialog.breadcrumbs.clear();
    
    let mut current = dialog.current_path.clone();
    dialog.breadcrumbs.push(current.clone());
    
    while let Some(parent) = current.parent() {
        current = parent.to_path_buf();
        dialog.breadcrumbs.insert(0, current.clone());
    }
}
