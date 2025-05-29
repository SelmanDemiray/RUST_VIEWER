use eframe::egui;
use crate::app::App;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("file_panel").show(ctx, |ui| {
        ui.heading("Project Files");
        ui.separator();
        
        if app.project.files.is_empty() {
            ui.label("No project loaded.");
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for file in &app.project.files {
                    let is_selected = app.selected_file.as_ref() == Some(file);
                    if ui.selectable_label(is_selected, file).clicked() {
                        app.selected_file = Some(file.clone());
                    }
                }
            });
        }
    });
}
