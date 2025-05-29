use eframe::egui;

pub fn render_code_editor(ui: &mut egui::Ui, file_name: &str, content: &str) {
    ui.vertical(|ui| {
        // Header with file name
        ui.horizontal(|ui| {
            ui.heading("Code Editor");
            ui.separator();
            ui.label(format!("File: {}", file_name));
        });
        
        ui.separator();
        
        // Code content area
        egui::ScrollArea::both()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                render_code_content(ui, content);
            });
    });
}

fn render_code_content(ui: &mut egui::Ui, content: &str) {
    // Split content into lines for better rendering
    let lines: Vec<&str> = content.lines().collect();
    
    ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
    
    // Create a table-like layout for line numbers and content
    egui::Grid::new("code_grid")
        .striped(true)
        .spacing([0.0, 0.0])
        .show(ui, |ui| {
            for (line_num, line) in lines.iter().enumerate() {
                // Line number column
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::Center),
                    |ui| {
                        ui.label(
                            egui::RichText::new(format!("{:4}", line_num + 1))
                                .color(egui::Color32::from_gray(128))
                                .monospace()
                        );
                    }
                );
                
                // Code content column
                ui.with_layout(
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        render_syntax_highlighted_line(ui, line);
                    }
                );
                
                ui.end_row();
            }
        });
}

fn render_syntax_highlighted_line(ui: &mut egui::Ui, line: &str) {
    // Basic syntax highlighting for Rust
    let mut current_pos = 0;
    let line_chars: Vec<char> = line.chars().collect();
    
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        
        while current_pos < line_chars.len() {
            let (token, color, length) = get_next_token(&line_chars, current_pos);
            
            ui.label(
                egui::RichText::new(token)
                    .color(color)
                    .monospace()
            );
            
            current_pos += length;
        }
    });
}

fn get_next_token(chars: &[char], start: usize) -> (String, egui::Color32, usize) {
    if start >= chars.len() {
        return (String::new(), egui::Color32::WHITE, 0);
    }
    
    let current_char = chars[start];
    
    // Skip whitespace
    if current_char.is_whitespace() {
        let mut end = start;
        while end < chars.len() && chars[end].is_whitespace() {
            end += 1;
        }
        return (
            chars[start..end].iter().collect(),
            egui::Color32::WHITE,
            end - start
        );
    }
    
    // Comments
    if start + 1 < chars.len() && chars[start] == '/' && chars[start + 1] == '/' {
        return (
            chars[start..].iter().collect(),
            egui::Color32::from_rgb(120, 120, 120),
            chars.len() - start
        );
    }
    
    // String literals
    if current_char == '"' {
        let mut end = start + 1;
        let mut escaped = false;
        while end < chars.len() {
            if escaped {
                escaped = false;
            } else if chars[end] == '\\' {
                escaped = true;
            } else if chars[end] == '"' {
                end += 1;
                break;
            }
            end += 1;
        }
        return (
            chars[start..end].iter().collect(),
            egui::Color32::from_rgb(120, 200, 120),
            end - start
        );
    }
    
    // Keywords and identifiers
    if current_char.is_alphabetic() || current_char == '_' {
        let mut end = start;
        while end < chars.len() && (chars[end].is_alphanumeric() || chars[end] == '_') {
            end += 1;
        }
        
        let token: String = chars[start..end].iter().collect();
        let color = get_keyword_color(&token);
        
        return (token, color, end - start);
    }
    
    // Numbers
    if current_char.is_numeric() {
        let mut end = start;
        while end < chars.len() && (chars[end].is_numeric() || chars[end] == '.') {
            end += 1;
        }
        return (
            chars[start..end].iter().collect(),
            egui::Color32::from_rgb(200, 150, 100),
            end - start
        );
    }
    
    // Operators and punctuation
    let operators = ['=', '+', '-', '*', '/', '<', '>', '!', '&', '|', '^', '%'];
    if operators.contains(&current_char) {
        return (
            current_char.to_string(),
            egui::Color32::from_rgb(200, 200, 100),
            1
        );
    }
    
    // Default: single character
    (
        current_char.to_string(),
        egui::Color32::WHITE,
        1
    )
}

fn get_keyword_color(token: &str) -> egui::Color32 {
    match token {
        // Rust keywords
        "fn" | "let" | "mut" | "const" | "static" | "if" | "else" | "match" | "for" | "while" | 
        "loop" | "break" | "continue" | "return" | "struct" | "enum" | "trait" | "impl" | 
        "mod" | "use" | "pub" | "crate" | "super" | "self" | "Self" | "where" | "type" | 
        "unsafe" | "extern" | "async" | "await" | "move" | "ref" | "in" | "as" => {
            egui::Color32::from_rgb(100, 150, 255)
        },
        
        // Types
        "bool" | "char" | "str" | "u8" | "u16" | "u32" | "u64" | "u128" | "usize" |
        "i8" | "i16" | "i32" | "i64" | "i128" | "isize" | "f32" | "f64" |
        "String" | "Vec" | "Option" | "Result" | "Box" | "Rc" | "Arc" => {
            egui::Color32::from_rgb(255, 150, 100)
        },
        
        // Constants
        "true" | "false" | "None" | "Some" | "Ok" | "Err" => {
            egui::Color32::from_rgb(255, 200, 100)
        },
        
        _ => egui::Color32::WHITE
    }
}
