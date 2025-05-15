use eframe::egui;
use crate::project::Project;
use crate::parser::{ElementType, RelationshipType};
use std::collections::HashMap;

pub struct VisualizationState {
    zoom: f32,
    pan_offset: egui::Vec2,
    dragging: bool,
    last_pointer_pos: Option<egui::Pos2>,
    selected_element: Option<String>,
    layout_type: LayoutType,
    show_all_relationships: bool,
    animation_progress: f32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LayoutType {
    Grid,
    Circular,
    Tree,
    ForceDirected,
}

impl Default for VisualizationState {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_offset: egui::Vec2::ZERO,
            dragging: false,
            last_pointer_pos: None,
            selected_element: None,
            layout_type: LayoutType::Grid,
            show_all_relationships: true,
            animation_progress: 0.0,
        }
    }
}

impl VisualizationState {
    pub fn update_animation(&mut self, ctx: &egui::Context) {
        if self.animation_progress < 1.0 {
            self.animation_progress += 0.05;
            if self.animation_progress > 1.0 {
                self.animation_progress = 1.0;
            }
            ctx.request_repaint();
        }
    }
}

pub fn render_visualization(ui: &mut egui::Ui, project: &Project, state: &mut VisualizationState) {
    // Get the available view rect
    let available_size = ui.available_size();
    
    // Toolbar for visualization controls
    ui.horizontal(|ui| {
        ui.label("Zoom:");
        if ui.button("➕").clicked() {
            state.zoom *= 1.2;
        }
        if ui.button("➖").clicked() {
            state.zoom *= 0.8;
        }
        if ui.button("🔄 Reset").clicked() {
            state.zoom = 1.0;
            state.pan_offset = egui::Vec2::ZERO;
        }
        
        ui.separator();
        
        ui.label("Layout:");
        if ui.selectable_label(state.layout_type == LayoutType::Grid, "Grid").clicked() {
            state.layout_type = LayoutType::Grid;
            state.animation_progress = 0.0;
        }
        if ui.selectable_label(state.layout_type == LayoutType::Circular, "Circular").clicked() {
            state.layout_type = LayoutType::Circular;
            state.animation_progress = 0.0;
        }
        if ui.selectable_label(state.layout_type == LayoutType::Tree, "Tree").clicked() {
            state.layout_type = LayoutType::Tree;
            state.animation_progress = 0.0;
        }
        if ui.selectable_label(state.layout_type == LayoutType::ForceDirected, "Force").clicked() {
            state.layout_type = LayoutType::ForceDirected;
            state.animation_progress = 0.0;
        }
        
        ui.separator();
        
        ui.checkbox(&mut state.show_all_relationships, "Show all connections");
    });
    
    // Create a visualization canvas
    egui::ScrollArea::both().show(ui, |ui| {
        // Allocate a large enough painting area
        let (response, painter) = ui.allocate_painter(
            available_size,
            egui::Sense::click_and_drag(),
        );
        
        let rect = response.rect;
        
        // Draw background
        painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(20, 20, 30));
        
        if project.files.is_empty() {
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Open a project to visualize its code",
                egui::FontId::default(),
                egui::Color32::WHITE,
            );
            return;
        }
        
        // Handle input (zoom and pan)
        if response.dragged() && response.drag_delta().length() > 0.0 {
            state.dragging = true;
            if let Some(last_pos) = state.last_pointer_pos {
                let delta = response.hover_pos().unwrap_or(last_pos) - last_pos;
                state.pan_offset += delta;
            }
            state.last_pointer_pos = response.hover_pos();
        } else {
            state.dragging = false;
            state.last_pointer_pos = None;
        }
        
        if response.clicked() {
            // Check if clicked on any element
            // This will be implemented when we draw the elements
            state.selected_element = None;
        }
        
        if let Some(hover_pos) = response.hover_pos() {
            // Handle scroll for zoom
            ui.input(|input| {
                let scroll = input.scroll_delta.y;
                if scroll.abs() > 1.0 {
                    let old_zoom = state.zoom;
                    if scroll > 0.0 {
                        state.zoom *= 1.1;
                    } else {
                        state.zoom *= 0.9;
                    }
                    state.zoom = state.zoom.clamp(0.2, 5.0);
                    
                    // Adjust pan offset to zoom toward cursor position
                    if state.zoom != old_zoom {
                        let zoom_center = hover_pos;
                        let zoom_delta = state.zoom / old_zoom;
                        let center_to_point = zoom_center - rect.center() - state.pan_offset;
                        let offset = center_to_point - center_to_point * zoom_delta;
                        state.pan_offset += offset;
                    }
                }
            });
        }
        
        // Create node positions based on selected layout
        let (file_positions, element_positions) = calculate_positions(
            project,
            state.layout_type,
            state.zoom,
            rect.center() + state.pan_offset,
            state.animation_progress
        );
        
        // Create a copy of the selected_element to avoid borrow issues
        let selected_element_clone = state.selected_element.clone();
        let selected_element_ref = selected_element_clone.as_ref();
        
        // Draw relationships first (so they're behind the nodes)
        draw_relationships(
            project,
            &painter,
            &element_positions,
            selected_element_ref,
            state.show_all_relationships
        );
        
        // Draw files and elements
        draw_files_and_elements(
            project,
            &painter,
            &file_positions,
            &element_positions,
            selected_element_ref,
            &response,
            state
        );
        
        // Draw minimap
        draw_minimap(ui, rect, &file_positions, &element_positions, state);
        
        // Status bar with info
        draw_status_bar(ui, project, state);
    });
    
    // Update animation for smooth transitions
    state.update_animation(ui.ctx());
}

fn calculate_positions(
    project: &Project,
    layout_type: LayoutType,
    zoom: f32,
    center: egui::Pos2,
    animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();
    
    // Calculate positions based on layout type
    match layout_type {
        LayoutType::Grid => {
            // Position files in a grid
            let mut x = -300.0 * zoom;
            let mut y = -200.0 * zoom;
            let file_spacing_x = 200.0 * zoom;
            let file_spacing_y = 150.0 * zoom;
            
            for (i, file) in project.files.iter().enumerate() {
                let pos = center + egui::vec2(x, y);
                file_positions.insert(file.clone(), pos);
                
                // Move to next position
                x += file_spacing_x;
                if (i + 1) % 3 == 0 {
                    x = -300.0 * zoom;
                    y += file_spacing_y;
                }
            }
            
            // Position code elements below their files
            for element in &project.elements {
                if let Some(file_pos) = file_positions.get(&element.file_path) {
                    let element_offset = egui::vec2(
                        (element.id.len() as f32 % 3.0 - 1.0) * 50.0 * zoom,
                        70.0 * zoom
                    );
                    element_positions.insert(element.id.clone(), *file_pos + element_offset);
                }
            }
        },
        LayoutType::Circular => {
            // Position files in a circle
            let file_count = project.files.len() as f32;
            let radius = 250.0 * zoom;
            
            for (i, file) in project.files.iter().enumerate() {
                let angle = (i as f32 / file_count) * std::f32::consts::TAU;
                let x = radius * angle.cos();
                let y = radius * angle.sin();
                let pos = center + egui::vec2(x, y);
                file_positions.insert(file.clone(), pos);
            }
            
            // Position elements in smaller circles around their files
            for element in &project.elements {
                if let Some(file_pos) = file_positions.get(&element.file_path) {
                    let element_count = project.elements
                        .iter()
                        .filter(|e| e.file_path == element.file_path)
                        .count() as f32;
                    
                    let element_index = project.elements
                        .iter()
                        .filter(|e| e.file_path == element.file_path)
                        .position(|e| e.id == element.id)
                        .unwrap_or(0) as f32;
                    
                    let angle = (element_index / element_count) * std::f32::consts::TAU;
                    let small_radius = 60.0 * zoom;
                    let x = small_radius * angle.cos();
                    let y = small_radius * angle.sin();
                    
                    element_positions.insert(element.id.clone(), *file_pos + egui::vec2(x, y));
                }
            }
        },
        LayoutType::Tree => {
            // Position files in a hierarchical tree
            let max_depth = 3;
            let width = 600.0 * zoom;
            let height = 500.0 * zoom;
            
            // Root at the top
            file_positions.insert(
                project.files.first().cloned().unwrap_or_default(),
                center + egui::vec2(0.0, -height/2.0)
            );
            
            // Other files in levels below
            for (i, file) in project.files.iter().skip(1).enumerate() {
                let depth = (i / 3) + 1;
                if depth >= max_depth { continue; }
                
                let items_at_level = 3.min(project.files.len() - 1 - (i / 3) * 3);
                let position_in_level = i % 3;
                
                let x = width * ((position_in_level as f32 + 0.5) / items_at_level as f32 - 0.5);
                let y = (depth as f32 * height) / max_depth as f32 - height/2.0;
                
                file_positions.insert(file.clone(), center + egui::vec2(x, y));
            }
            
            // Position elements as leaf nodes
            for element in &project.elements {
                if let Some(file_pos) = file_positions.get(&element.file_path) {
                    let elements_in_file = project.elements
                        .iter()
                        .filter(|e| e.file_path == element.file_path)
                        .count();
                    
                    let element_index = project.elements
                        .iter()
                        .filter(|e| e.file_path == element.file_path)
                        .position(|e| e.id == element.id)
                        .unwrap_or(0);
                    
                    let spacing = 120.0 * zoom / elements_in_file.max(1) as f32;
                    let offset = spacing * (element_index as f32 - (elements_in_file as f32 - 1.0) / 2.0);
                    
                    element_positions.insert(
                        element.id.clone(),
                        *file_pos + egui::vec2(offset, 60.0 * zoom)
                    );
                }
            }
        },
        LayoutType::ForceDirected => {
            // Simple force-directed layout
            // For a real implementation, you'd use a proper force-directed algorithm
            
            // Start with a grid layout
            let mut x = -300.0 * zoom;
            let mut y = -200.0 * zoom;
            let file_spacing = 200.0 * zoom;
            
            for (i, file) in project.files.iter().enumerate() {
                let pos = center + egui::vec2(x, y);
                file_positions.insert(file.clone(), pos);
                
                // Move to next position
                x += file_spacing;
                if (i + 1) % 3 == 0 {
                    x = -300.0 * zoom;
                    y += file_spacing;
                }
            }
            
            // Position code elements with some randomization
            for element in &project.elements {
                if let Some(file_pos) = file_positions.get(&element.file_path) {
                    // Pseudo-random but deterministic positions
                    let hash = element.id.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32));
                    let angle = (hash % 628) as f32 / 100.0;  // 0 to 2π
                    let distance = 50.0 + (hash % 50) as f32;
                    
                    let x = file_pos.x + distance * zoom * angle.cos();
                    let y = file_pos.y + distance * zoom * angle.sin();
                    
                    element_positions.insert(element.id.clone(), egui::pos2(x, y));
                }
            }
        }
    }
    
    // If still in animation, interpolate between previous positions
    if animation_progress < 1.0 {
        // Implement animation logic here if needed
        // For now, we'll just use the final positions
    }
    
    (file_positions, element_positions)
}

fn draw_relationships(
    project: &Project,
    painter: &egui::Painter,
    element_positions: &HashMap<String, egui::Pos2>,
    selected_id: Option<&String>,
    show_all_relationships: bool
) {
    for rel in &project.relationships {
        let should_draw = show_all_relationships || 
                         selected_id.map_or(false, |id| *id == rel.source_id || *id == rel.target_id);
        
        if should_draw {
            if let (Some(source_pos), Some(target_pos)) = (
                element_positions.get(&rel.source_id),
                element_positions.get(&rel.target_id),
            ) {
                let is_highlighted = selected_id.map_or(false, |id| *id == rel.source_id || *id == rel.target_id);
                
                let color = match rel.relationship_type {
                    RelationshipType::Calls => egui::Color32::from_rgb(220, 220, 50),
                    RelationshipType::Imports => egui::Color32::from_rgb(50, 220, 220),
                    RelationshipType::Implements => egui::Color32::from_rgb(220, 50, 220),
                    RelationshipType::Contains => egui::Color32::from_rgb(150, 150, 150),
                };
                
                // Draw thicker lines for highlighted relationships
                let stroke_width = if is_highlighted { 2.5 } else { 1.0 };
                
                // Draw the arrow
                painter.line_segment(
                    [*source_pos, *target_pos],
                    egui::Stroke::new(stroke_width, color),
                );
                
                // Draw arrowhead
                let dir = (*target_pos - *source_pos).normalized();
                let tip = *target_pos - dir * 10.0;
                
                // Calculate perpendicular points for arrow head
                let perp = egui::vec2(-dir.y, dir.x) * 4.0;
                let arrow_left = tip - dir * 8.0 + perp;
                let arrow_right = tip - dir * 8.0 - perp;
                
                // Draw arrow head as a triangle - FIXED METHOD
                let triangle_points = vec![
                    *target_pos,
                    arrow_left,
                    arrow_right,
                ];
                
                painter.add(egui::Shape::convex_polygon(
                    triangle_points,
                    color,
                    egui::Stroke::NONE, // No stroke outline
                ));
                
                // If highlighted, add a label for the relationship type
                if is_highlighted {
                    // Fix midpoint calculation by properly converting between Pos2 and Vec2
                    let midpoint = egui::pos2(
                        (source_pos.x + target_pos.x) * 0.5,
                        (source_pos.y + target_pos.y) * 0.5
                    );
                    
                    let rel_type = format!("{:?}", rel.relationship_type);
                    
                    painter.text(
                        midpoint,
                        egui::Align2::CENTER_CENTER,
                        &rel_type,
                        egui::FontId::proportional(10.0),
                        egui::Color32::WHITE,
                    );
                }
            }
        }
    }
}

fn draw_files_and_elements(
    project: &Project,
    painter: &egui::Painter,
    file_positions: &HashMap<String, egui::Pos2>,
    element_positions: &HashMap<String, egui::Pos2>,
    selected_id: Option<&String>,
    response: &egui::Response,
    state: &mut VisualizationState
) {
    // Draw files
    for (file_path, pos) in file_positions {
        let file_name = file_path.split('/').last().unwrap_or(file_path);
        let file_size = egui::vec2(120.0, 40.0) * state.zoom;
        let file_rect = egui::Rect::from_center_size(*pos, file_size);
        
        // Draw shadow for 3D effect
        painter.rect_filled(
            file_rect.translate(egui::vec2(3.0, 3.0)),
            5.0,
            egui::Color32::from_rgba_unmultiplied(0, 0, 0, 100),
        );
        
        // Draw file background
        painter.rect_filled(
            file_rect,
            5.0 * state.zoom,
            egui::Color32::from_rgb(70, 70, 120),
        );
        
        // Draw file name
        painter.text(
            file_rect.center(),
            egui::Align2::CENTER_CENTER,
            file_name,
            egui::FontId::proportional(14.0 * state.zoom),
            egui::Color32::WHITE,
        );
        
        // Check if clicked
        if response.clicked() {
            if let Some(pointer_pos) = response.hover_pos() {
                if file_rect.contains(pointer_pos) {
                    // Select this file (for a real app, you'd want to store the selection)
                }
            }
        }
    }
    
    // Draw code elements
    for element in &project.elements {
        if let Some(pos) = element_positions.get(&element.id) {
            let is_selected = selected_id.map_or(false, |id| *id == element.id);
            
            let color = match element.element_type {
                ElementType::Function => egui::Color32::from_rgb(120, 200, 120),
                ElementType::Module => egui::Color32::from_rgb(200, 120, 120),
                ElementType::Struct => egui::Color32::from_rgb(120, 120, 200),
                _ => egui::Color32::from_rgb(170, 170, 170),
            };
            
            let element_size = egui::vec2(90.0, 30.0) * state.zoom;
            let element_rect = egui::Rect::from_center_size(*pos, element_size);
            
            // Draw shadow for 3D effect
            painter.rect_filled(
                element_rect.translate(egui::vec2(2.0, 2.0)),
                3.0 * state.zoom,
                egui::Color32::from_rgba_unmultiplied(0, 0, 0, 80),
            );
            
            // Draw element background with highlight for selected elements
            let bg_color = if is_selected {
                egui::Color32::from_rgb(255, 255, 180)
            } else {
                color
            };
            
            painter.rect_filled(
                element_rect,
                3.0 * state.zoom,
                bg_color,
            );
            
            // Draw element name
            let text_color = if is_selected {
                egui::Color32::BLACK
            } else {
                egui::Color32::from_rgb(30, 30, 30)
            };
            
            painter.text(
                element_rect.center(),
                egui::Align2::CENTER_CENTER,
                &element.name,
                egui::FontId::proportional(12.0 * state.zoom),
                text_color,
            );
            
            // Add a small icon indicating the element type
            let icon = match element.element_type {
                ElementType::Function => "ƒ",
                ElementType::Module => "□",
                ElementType::Struct => "§",
                ElementType::Enum => "≡",
                ElementType::Trait => "T",
                ElementType::Impl => "I",
            };
            
            painter.text(
                element_rect.left_top() + egui::vec2(5.0, 5.0) * state.zoom,
                egui::Align2::LEFT_TOP,
                icon,
                egui::FontId::proportional(10.0 * state.zoom),
                text_color,
            );
            
            // Check if clicked
            if response.clicked() {
                if let Some(pointer_pos) = response.hover_pos() {
                    if element_rect.contains(pointer_pos) {
                        state.selected_element = Some(element.id.clone());
                    }
                }
            }
        }
    }
}

fn draw_minimap(
    ui: &mut egui::Ui,
    main_rect: egui::Rect,
    file_positions: &HashMap<String, egui::Pos2>,
    element_positions: &HashMap<String, egui::Pos2>,
    state: &mut VisualizationState
) {
    // Calculate minimap size and position
    let minimap_size = egui::vec2(150.0, 100.0);
    let minimap_pos = main_rect.right_bottom() - minimap_size - egui::vec2(10.0, 10.0);
    let minimap_rect = egui::Rect::from_min_size(minimap_pos, minimap_size);
    
    // Draw minimap background
    ui.painter().rect_filled(
        minimap_rect,
        5.0,
        egui::Color32::from_rgba_unmultiplied(30, 30, 40, 200),
    );
    
    // Calculate bounds of all elements for scaling
    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    
    for pos in file_positions.values() {
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
    }
    
    for pos in element_positions.values() {
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);
    }
    
    // Add some padding
    min_x -= 50.0;
    min_y -= 50.0;
    max_x += 50.0;
    max_y += 50.0;
    
    // Function to map a position to minimap coordinates
    let to_minimap = |pos: &egui::Pos2| {
        let normalized_x = (pos.x - min_x) / (max_x - min_x);
        let normalized_y = (pos.y - min_y) / (max_y - min_y);
        
        egui::pos2(
            minimap_rect.min.x + normalized_x * minimap_rect.width(),
            minimap_rect.min.y + normalized_y * minimap_rect.height(),
        )
    };
    
    // Draw files and elements on minimap
    for pos in file_positions.values() {
        let minimap_pos = to_minimap(pos);
        ui.painter().circle_filled(
            minimap_pos,
            4.0,
            egui::Color32::from_rgb(100, 100, 180),
        );
    }
    
    for pos in element_positions.values() {
        let minimap_pos = to_minimap(pos);
        ui.painter().circle_filled(
            minimap_pos,
            2.0,
            egui::Color32::from_rgb(180, 180, 100),
        );
    }
    
    // Draw viewport rectangle
    let viewport_min_x = (main_rect.min.x - state.pan_offset.x - min_x) / (max_x - min_x);
    let viewport_min_y = (main_rect.min.y - state.pan_offset.y - min_y) / (max_y - min_y);
    let viewport_max_x = (main_rect.max.x - state.pan_offset.x - min_x) / (max_x - min_x);
    let viewport_max_y = (main_rect.max.y - state.pan_offset.y - min_y) / (max_y - min_y);
    
    let viewport_rect = egui::Rect::from_min_max(
        egui::pos2(
            minimap_rect.min.x + viewport_min_x * minimap_rect.width(),
            minimap_rect.min.y + viewport_min_y * minimap_rect.height(),
        ),
        egui::pos2(
            minimap_rect.min.x + viewport_max_x * minimap_rect.width(),
            minimap_rect.min.y + viewport_max_y * minimap_rect.height(),
        ),
    );
    
    ui.painter().rect_stroke(
        viewport_rect,
        0.0,
        egui::Stroke::new(1.0, egui::Color32::WHITE),
    );
}

fn draw_status_bar(ui: &mut egui::Ui, project: &Project, state: &VisualizationState) {
    // Draw status bar at the bottom
    ui.allocate_ui_at_rect(
        egui::Rect::from_min_size(
            ui.min_rect().left_bottom() - egui::vec2(0.0, 20.0),
            egui::vec2(ui.available_width(), 20.0),
        ),
        |ui| {
            ui.horizontal(|ui| {
                ui.label(format!("Project: {}", project.project_path.as_deref().unwrap_or("No project loaded")));
                ui.separator();
                ui.label(format!("Files: {}", project.files.len()));
                ui.separator();
                ui.label(format!("Elements: {}", project.elements.len()));
                ui.separator();
                ui.label(format!("Relationships: {}", project.relationships.len()));
                ui.separator();
                
                // Show selected element info
                if let Some(id) = &state.selected_element {
                    if let Some(element) = project.elements.iter().find(|e| e.id == *id) {
                        ui.label(format!(
                            "Selected: {} ({:?}) in {}",
                            element.name,
                            element.element_type,
                            element.file_path
                        ));
                    }
                }
            });
        },
    );
}
