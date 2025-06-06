use eframe::egui;
use crate::simple_dialog;

pub struct FileDialog {
    dialog: simple_dialog::SimpleFileDialog,
    is_open: bool,
}

impl FileDialog {
    pub fn new() -> Self {
        Self {
            dialog: simple_dialog::SimpleFileDialog::new(),
            is_open: true,
        }
    }
    
    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        let mut result = None;
        
        if self.is_open {
            egui::Window::new("Select Project Directory")
                .collapsible(false)
                .resizable(true)
                .default_width(500.0)
                .default_height(400.0)
                .show(ctx, |ui| {
                    if let Some(path) = self.dialog.show(ui) {
                        if path.is_empty() {
                            // User cancelled
                            self.is_open = false;
                            result = None;
                        } else {
                            // User selected a folder
                            self.is_open = false;
                            result = Some(path);
                        }
                    }
                });
        }
        
        result
    }
    
    pub fn is_open(&self) -> bool {
        self.is_open
    }
}

// Placeholder for native file dialogs
// Currently using the simple_dialog module instead

pub struct NativeFileDialog;

impl NativeFileDialog {
    pub fn new() -> Self {
        Self
    }
}
