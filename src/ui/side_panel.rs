use eframe::egui;
use crate::app::state::App;

pub fn render(app: &mut App, ui: &mut egui::Ui) {
    ui.heading("Project Files");
    
    if let Some(ref project) = app.project {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for file in &project.files {
                if ui.selectable_label(
                    app.selected_file.as_ref() == Some(file),
                    file
                ).clicked() {
                    app.selected_file = Some(file.clone());
                }
            }
        });
    } else {
        ui.label("No project loaded");
    }
}
