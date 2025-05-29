pub mod top_panel;
pub mod side_panel;
pub mod central_panel;

use eframe::egui;

// Helper function to create consistent section headings
#[allow(dead_code)]
pub fn section_heading(ui: &mut egui::Ui, text: &str) {
    ui.add_space(8.0);
    ui.heading(text);
    ui.separator();
    ui.add_space(4.0);
}

// Helper to create a collapsing section with content
#[allow(dead_code)]
pub fn collapsing_section<R>(
    ui: &mut egui::Ui, 
    title: &str, 
    default_open: bool, 
    add_contents: impl FnOnce(&mut egui::Ui) -> R
) -> (bool, Option<R>) {
    ui.add_space(2.0);
    
    // In egui 0.21.0, we need to use the collapsing method and its built-in state management
    let id = ui.make_persistent_id(title);
    
    // Create the collapsing section with the default_open parameter
    let collapsing = egui::collapsing_header::CollapsingHeader::new(title)
        .id_source(id)
        .default_open(default_open);
    
    // Show the header and get the response
    let header_response = collapsing.show(ui, |ui| {
        add_contents(ui)
    });
    
    ui.add_space(2.0);
    
    // Return if the header was clicked and the content (if it was shown)
    (header_response.header_response.clicked(), header_response.body_returned)
}

// Display a status message in a consistent format
#[allow(dead_code)]
pub fn status_message(ui: &mut egui::Ui, message: &str, is_error: bool) {
    let color = if is_error {
        egui::Color32::RED
    } else {
        egui::Color32::GREEN
    };
    
    ui.horizontal(|ui| {
        ui.label(egui::RichText::new("●").color(color));
        ui.label(message);
    });
}

// Create a styled button
#[allow(dead_code)]
pub fn styled_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(egui::Button::new(text).min_size(egui::vec2(120.0, 30.0)))
}

// Helper for rendering file paths in a more readable way
#[allow(dead_code)]
pub fn display_file_path(ui: &mut egui::Ui, path: &str, max_width: f32) -> egui::Response {
    let parts: Vec<&str> = path.split('/').collect();
    let mut display_path = path.to_string();
    
    // If path is long, abbreviate middle parts
    if ui.available_width() < max_width && parts.len() > 3 {
        display_path = format!("{}/.../{}", 
            parts[0],
            parts[parts.len() - 1]
        );
    }
    
    ui.label(display_path).on_hover_text(path)
}
