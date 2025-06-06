use eframe::egui;
use crate::simple_dialog::SimpleFileDialog;
use super::view_modes::ViewMode;
use std::sync::Mutex;

pub fn render(dialog: &mut SimpleFileDialog, ui: &mut egui::Ui) -> Option<String> {
    // Use the existing show method directly instead of trying to access private fields
    dialog.show(ui)
}

pub fn render_file_dialog(ui: &mut egui::Ui) -> Option<String> {
    let mut result: Option<String> = None;
    
    ui.label("File Dialog Placeholder");
    if ui.button("Select File").clicked() {
        result = Some("selected_file.rs".to_string());
    }
    
    result
}

// Dialog rendering logic
pub fn render_dialog(ui: &mut egui::Ui, title: &str, content: &str) -> bool {
    ui.label(title);
    ui.separator();
    ui.label(content);
    
    ui.button("OK").clicked()
}

pub fn render_view_mode(ui: &mut egui::Ui, view_mode: &mut ViewMode) {
    match view_mode {
        ViewMode::List => {
            ui.label("List view");
        }
        ViewMode::Grid => {
            ui.label("Grid view");
        }
    }
}

// Safe alternative using Mutex instead of unsafe static
static DIALOG_STATE: Mutex<Option<SimpleFileDialog>> = Mutex::new(None);

pub fn show_file_dialog(ui: &mut egui::Ui) -> Option<String> {
    let mut dialog_guard = DIALOG_STATE.lock().unwrap();
    
    // Initialize dialog if it doesn't exist
    if dialog_guard.is_none() {
        *dialog_guard = Some(SimpleFileDialog::new());
    }
    
    if let Some(ref mut dialog) = *dialog_guard {
        let result = dialog.show(ui);
        
        // If a selection was made or dialog was canceled, reset for next time
        if result.is_some() {
            *dialog_guard = None;
        }
        
        result
    } else {
        None
    }
}

pub fn show_folder_dialog_window(ctx: &egui::Context) -> Option<String> {
    let mut result = None;
    let mut should_close = false;
    
    egui::Window::new("Select Project Folder")
        .default_size([600.0, 400.0])
        .resizable(true)
        .show(ctx, |ui| {
            if let Some(path) = show_file_dialog(ui) {
                if path.is_empty() {
                    // User canceled
                    should_close = true;
                } else {
                    // User selected a folder
                    result = Some(path);
                    should_close = true;
                }
            }
        });
    
    if should_close {
        // Window will close automatically
    }
    
    result
}

// Safe alternative for global dialog state
static GLOBAL_DIALOG_STATE: Mutex<Option<(SimpleFileDialog, bool)>> = Mutex::new(None);

pub fn open_folder_dialog() {
    let mut state = GLOBAL_DIALOG_STATE.lock().unwrap();
    *state = Some((SimpleFileDialog::new(), true));
}

pub fn update_folder_dialog(ctx: &egui::Context) -> Option<String> {
    let mut state = GLOBAL_DIALOG_STATE.lock().unwrap();
    
    if let Some((ref mut dialog, ref mut is_open)) = *state {
        if !*is_open {
            return None;
        }
        
        let mut result = None;
        let mut should_close = false;
        
        egui::Window::new("Select Project Folder")
            .open(is_open)
            .default_size([600.0, 400.0])
            .resizable(true)
            .show(ctx, |ui| {
                if let Some(path) = dialog.show(ui) {
                    if path.is_empty() {
                        should_close = true;
                    } else {
                        result = Some(path);
                        should_close = true;
                    }
                }
            });
        
        if should_close || !*is_open {
            *state = None;
        }
        
        result
    } else {
        None
    }
}
