use eframe::egui;
use crate::parser::types::RelationshipType;
use crate::visualization::state::VisualizationState;

pub fn render_relationships(ui: &mut egui::Ui, state: &VisualizationState) {
    ui.heading("Relationships");
    
    egui::ScrollArea::vertical().show(ui, |ui| {
        for edge in &state.edges {
            ui.horizontal(|ui| {
                let color = match edge.relationship.relationship_type {
                    RelationshipType::Imports => egui::Color32::BLUE,
                    RelationshipType::Uses => egui::Color32::from_rgb(0, 255, 255), // CYAN
                    RelationshipType::Extends => egui::Color32::from_rgb(255, 165, 0), // ORANGE
                    RelationshipType::Implements => egui::Color32::GREEN,
                    RelationshipType::Contains => egui::Color32::YELLOW,
                    RelationshipType::Calls => egui::Color32::RED,
                    RelationshipType::Instantiates => egui::Color32::from_rgb(128, 0, 128), // PURPLE
                    RelationshipType::References => egui::Color32::from_rgb(255, 192, 203), // PINK
                    RelationshipType::DependsOn => egui::Color32::BROWN,
                    RelationshipType::AssociatedWith => egui::Color32::GRAY,
                };
                
                ui.colored_label(color, "→");
                ui.label(format!("{} → {}", edge.source, edge.target));
                ui.label(format!("({:?})", edge.relationship.relationship_type));
            });
        }
    });
}
