use eframe::egui;

pub struct TextEditor {
    content: String,
}

impl TextEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
    
    pub fn show(&mut self, ui: &mut egui::Ui) {
        ui.text_edit_multiline(&mut self.content);
    }
}
