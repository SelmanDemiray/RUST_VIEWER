use eframe::egui;
use crate::parser::types::ElementType;
use crate::visualization::state::VisualizationState;

pub fn render_nodes(ui: &mut egui::Ui, state: &VisualizationState) {
    ui.heading("Nodes");
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for node in &state.nodes {
            let color = match node.element.element_type {
                ElementType::Function => egui::Color32::LIGHT_BLUE,
                ElementType::Struct => egui::Color32::LIGHT_GREEN,
                ElementType::Enum => egui::Color32::LIGHT_YELLOW,
                ElementType::Trait => egui::Color32::LIGHT_RED,
                ElementType::Module => egui::Color32::LIGHT_GRAY,
                ElementType::Impl => egui::Color32::from_rgb(200, 150, 255),
            };
            
            ui.horizontal(|ui| {
                ui.colored_label(color, "●");
                ui.label(&node.element.name);
                ui.label(format!("({:?})", node.element.element_type));
            });
        }
    });
}
