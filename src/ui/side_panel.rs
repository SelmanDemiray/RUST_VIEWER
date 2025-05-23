use crate::app::App;
use eframe::egui;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::SidePanel::left("file_panel").show(ctx, |ui| {
        ui.heading("Files");
        
        for file_path in &app.project.files {
            let file_name = file_path.split('/').last().unwrap_or(file_path);
            if ui.selectable_label(
                Some(file_path) == app.selected_file.as_ref(),
                file_name
            ).clicked() {
                app.selected_file = Some(file_path.clone());
            }
        }
    });
}
