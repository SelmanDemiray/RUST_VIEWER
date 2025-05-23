use eframe::egui;

pub fn render(ui: &mut egui::Ui, file_path: &str, content: &str) {
    ui.heading(file_path);
    
    // Use a simple egui TextEdit instead of egui_code_editor to avoid dependencies
    let mut code = content.to_string();
    
    ui.add(
        egui::TextEdit::multiline(&mut code)
            .code_editor()
            .desired_rows(30)
            .font(egui::TextStyle::Monospace),
    );
    
    // In a real application, you would save changes back to the file
    // when desired by the user
}
