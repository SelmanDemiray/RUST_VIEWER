use crate::app::state::App;
use eframe::egui;

pub fn render_main_ui(app: &mut App, ctx: &egui::Context) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Open Project").clicked() {
                app.show_file_dialog = true;
            }
            
            ui.separator();
            
            if let Some(ref project) = app.project {
                ui.label(format!("Project: {}", project.name()));
            } else {
                ui.label("No project loaded");
            }
        });
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        if let Some(_project) = &app.project {
            app.visualization.render(ui);
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("Open a Rust project to get started");
            });
        }
    });
}
