pub mod force_directed;

use eframe::egui;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::{
    project::Project,
    visualization::state::LayoutType,
    LayoutSettings,
};

use force_directed::ForceDirectedLayout;

// Define missing types for layout calculations
#[derive(Debug, Clone)]
pub struct LayoutNode {
    pub id: String,
    pub position: egui::Pos2,
    pub size: f32,
}

#[derive(Debug, Clone)]
pub struct LayoutEdge {
    pub source: String,
    pub target: String,
    pub edge_type: EdgeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EdgeType {
    Contains,
    Uses,
    Implements,
    Calls,
}

// Global layout state
static LAYOUT_STATE: Mutex<Option<LayoutState>> = Mutex::new(None);

struct LayoutState {
    force_directed: ForceDirectedLayout,
    settings: LayoutSettings,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            force_directed: ForceDirectedLayout::new(),
            settings: LayoutSettings::default(),
        }
    }
}

pub fn reset_layout() {
    if let Ok(mut state) = LAYOUT_STATE.lock() {
        if let Some(ref mut layout_state) = *state {
            layout_state.force_directed.reset();
        }
    }
}

pub fn render_force_settings(ui: &mut egui::Ui) {
    if let Ok(mut state) = LAYOUT_STATE.lock() {
        let layout_state = state.get_or_insert_with(LayoutState::default);

        ui.add(
            egui::Slider::new(&mut layout_state.settings.force_strength, 0.01..=1.0)
                .text("Force Strength"),
        );
        ui.add(
            egui::Slider::new(&mut layout_state.settings.repulsion_strength, 10.0..=500.0)
                .text("Repulsion"),
        );
        ui.add(
            egui::Slider::new(&mut layout_state.settings.spring_length, 20.0..=150.0)
                .text("Spring Length"),
        );
        ui.add(
            egui::Slider::new(&mut layout_state.settings.damping, 0.1..=0.99)
                .text("Damping"),
        );
    }
}

pub fn calculate_positions(
    project: &Project,
    layout_type: &LayoutType,
    zoom: f32,
    center: egui::Pos2,
    animation_progress: f32,
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();

    if project.elements.is_empty() {
        return (file_positions, element_positions);
    }

    match layout_type {
        LayoutType::ForceDirected => {
            calculate_force_directed_positions(project, zoom, center, animation_progress, &mut element_positions);
        },
        LayoutType::Grid => {
            calculate_grid_positions(project, zoom, center, &mut element_positions);
        },
        LayoutType::Circular => {
            calculate_circular_positions(project, zoom, center, &mut element_positions);
        },
        LayoutType::Tree => {
            calculate_tree_positions(project, zoom, center, &mut element_positions);
        },
        LayoutType::Hierarchical => {
            calculate_hierarchical_positions(project, zoom, center, &mut element_positions);
        },
    }

    // Calculate file positions based on element positions
    calculate_file_positions(project, &element_positions, &mut file_positions);

    (file_positions, element_positions)
}

fn calculate_force_directed_positions(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    animation_progress: f32,
    element_positions: &mut HashMap<String, egui::Pos2>,
) {
    if let Ok(mut state) = LAYOUT_STATE.lock() {
        let layout_state = state.get_or_insert_with(LayoutState::default);

        // Initialize if needed
        if layout_state.force_directed.positions.is_empty() {
            let bounds = egui::Rect::from_center_size(center, egui::vec2(800.0 * zoom, 600.0 * zoom));
            layout_state.force_directed.initialize_positions(&project.elements, center, bounds);
        }

        // Update positions using force-directed algorithm
        if animation_progress < 1.0 {
            layout_state.force_directed.step(project, 0.016); // ~60fps
        }

        // Copy positions
        for (id, pos) in layout_state.force_directed.get_positions() {
            element_positions.insert(id.clone(), *pos);
        }
    }
}

fn calculate_grid_positions(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    element_positions: &mut HashMap<String, egui::Pos2>,
) {
    let grid_size = (project.elements.len() as f32).sqrt().ceil() as usize;
    let spacing = 80.0 * zoom;
    let start_x = center.x - (grid_size as f32 * spacing) / 2.0;
    let start_y = center.y - (grid_size as f32 * spacing) / 2.0;

    for (i, element) in project.elements.iter().enumerate() {
        let row = i / grid_size;
        let col = i % grid_size;
        let pos = egui::pos2(
            start_x + col as f32 * spacing,
            start_y + row as f32 * spacing,
        );
        element_positions.insert(element.id.clone(), pos);
    }
}

fn calculate_circular_positions(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    element_positions: &mut HashMap<String, egui::Pos2>,
) {
    let radius = 200.0 * zoom;
    let count = project.elements.len();

    for (i, element) in project.elements.iter().enumerate() {
        let angle = if count > 1 {
            2.0 * std::f32::consts::PI * i as f32 / count as f32
        } else {
            0.0
        };

        let pos = center + egui::vec2(
            radius * angle.cos(),
            radius * angle.sin(),
        );
        element_positions.insert(element.id.clone(), pos);
    }
}

fn calculate_tree_positions(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    element_positions: &mut HashMap<String, egui::Pos2>,
) {
    // Simple tree layout - group by file and arrange vertically
    let mut files: Vec<String> = project.files.clone();
    files.sort();

    let y_spacing = 100.0 * zoom;
    let x_spacing = 150.0 * zoom;

    for (file_idx, file) in files.iter().enumerate() {
        let file_elements: Vec<_> = project.elements.iter()
            .filter(|e| &e.file_path == file)
            .collect();

        let file_y = center.y + (file_idx as f32 - files.len() as f32 / 2.0) * y_spacing;

        for (elem_idx, element) in file_elements.iter().enumerate() {
            let pos = egui::pos2(
                center.x + (elem_idx as f32 - file_elements.len() as f32 / 2.0) * x_spacing,
                file_y,
            );
            element_positions.insert(element.id.clone(), pos);
        }
    }
}

fn calculate_hierarchical_positions(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    element_positions: &mut HashMap<String, egui::Pos2>,
) {
    // Group elements by type in layers
    use crate::parser::ElementType;

    let layers = [
        ElementType::Module,
        ElementType::Struct,
        ElementType::Trait,
        ElementType::Impl,
        ElementType::Function,
        ElementType::Enum,
    ];

    let layer_spacing = 120.0 * zoom;
    let element_spacing = 60.0 * zoom;

    for (layer_idx, element_type) in layers.iter().enumerate() {
        let layer_elements: Vec<_> = project.elements.iter()
            .filter(|e| &e.element_type == element_type)
            .collect();

        if layer_elements.is_empty() {
            continue;
        }

        let layer_y = center.y + (layer_idx as f32 - layers.len() as f32 / 2.0) * layer_spacing;

        for (elem_idx, element) in layer_elements.iter().enumerate() {
            let pos = egui::pos2(
                center.x + (elem_idx as f32 - layer_elements.len() as f32 / 2.0) * element_spacing,
                layer_y,
            );
            element_positions.insert(element.id.clone(), pos);
        }
    }
}

fn calculate_file_positions(
    project: &Project,
    element_positions: &HashMap<String, egui::Pos2>,
    file_positions: &mut HashMap<String, egui::Pos2>,
) {
    for file in &project.files {
        let file_elements: Vec<_> = project.elements.iter()
            .filter(|e| &e.file_path == file)
            .collect();

        if !file_elements.is_empty() {
            let mut sum = egui::Vec2::ZERO;
            let mut count = 0;

            for element in file_elements {
                if let Some(pos) = element_positions.get(&element.id) {
                    sum += pos.to_vec2();
                    count += 1;
                }
            }

            if count > 0 {
                let center = egui::pos2(sum.x / count as f32, sum.y / count as f32);
                file_positions.insert(file.clone(), center);
            }
        }
    }
}
