use crate::app::{App, ViewMode};
use eframe::egui;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open Project").clicked() {
                app.show_dialog = true;
            }
            
            ui.separator();
            
            if ui.selectable_label(matches!(app.view_mode, ViewMode::Visualization), "Visualization").clicked() {
                app.view_mode = ViewMode::Visualization;
            }
            
            if ui.selectable_label(matches!(app.view_mode, ViewMode::Editor), "Editor").clicked() {
                app.view_mode = ViewMode::Editor;
            }
        });
    });
}
