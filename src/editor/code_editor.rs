use eframe::egui;

pub struct CodeEditor {
    content: String,
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = content.to_string();
    }
    
    pub fn get_content(&self) -> String {
        self.content.clone()
    }

    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::both()
            .show(ui, |ui| {
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.content)
                        .desired_width(f32::INFINITY)
                        .min_size(egui::vec2(400.0, 300.0))
                );
                
                // Show line count and basic stats
                if response.has_focus() {
                    ui.horizontal(|ui| {
                        ui.label(format!("Lines: {}", self.content.lines().count()));
                        ui.label(format!("Characters: {}", self.content.len()));
                    });
                }
            });
    }
}
