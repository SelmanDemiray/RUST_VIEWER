use crate::app::state::{App, ViewMode};
use eframe::egui;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Rust Code Visualizer");

            ui.separator();

            if ui.selectable_label(app.view_mode == ViewMode::Visualization, "Visualization").clicked() {
                app.view_mode = ViewMode::Visualization;
            }
            if ui.selectable_label(app.view_mode == ViewMode::Editor, "Editor").clicked() {
                app.view_mode = ViewMode::Editor;
            }

            ui.separator();

            if ui.button("Open Project").clicked() {
                app.show_file_dialog = true;
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(&app.status_message);
            });
        });
    });
}
