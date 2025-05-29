use crate::app::{App, ViewMode};
use eframe::egui;

pub fn render(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open Project").clicked() {
                    app.show_dialog = true;
                    ui.close_menu();
                }
                
                ui.separator();
                
                if ui.button("Exit").clicked() {
                    // Use the correct method for egui 0.21
                    std::process::exit(0);
                }
            });
            
            ui.separator();
            
            if ui.selectable_label(app.view_mode == ViewMode::Visualization, "Visualization").clicked() {
                app.view_mode = ViewMode::Visualization;
            }
            
            if ui.selectable_label(app.view_mode == ViewMode::Editor, "Editor").clicked() {
                app.view_mode = ViewMode::Editor;
            }
        });
    });
}
