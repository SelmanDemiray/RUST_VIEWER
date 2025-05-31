use crate::project::Project;
use eframe::egui;
use std::collections::{HashMap, BTreeMap};

pub fn calculate(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    _animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();
    
    if project.files.is_empty() {
        return (file_positions, element_positions);
    }

    // Build directory tree
    let mut directory_tree: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut all_directories = std::collections::HashSet::new();
    
    for file in &project.files {
        let parts: Vec<&str> = file.split('/').collect();
        let mut current_path = String::new();
        
        for (i, part) in parts.iter().enumerate() {
            if i == parts.len() - 1 {
                // This is the file
                directory_tree.entry(current_path.clone()).or_default().push(file.clone());
            } else {
                // This is a directory
                if i > 0 {
                    current_path.push('/');
                }
                current_path.push_str(part);
                all_directories.insert(current_path.clone());
            }
        }
    }

    // Calculate layout parameters
    let level_height = 180.0 * zoom;
    let node_spacing = 250.0 * zoom;
    let max_depth = all_directories.iter()
        .map(|dir| dir.split('/').count())
        .max()
        .unwrap_or(1);

    // Position directories by depth
    let mut directories_by_depth: Vec<Vec<String>> = vec![Vec::new(); max_depth + 1];
    
    // Root level (files in root)
    if let Some(root_files) = directory_tree.get("") {
        if !root_files.is_empty() {
            directories_by_depth[0].push("".to_string());
        }
    }
    
    // Other directories
    for dir in &all_directories {
        let depth = dir.split('/').count();
        if depth <= max_depth {
            directories_by_depth[depth].push(dir.clone());
        }
    }

    // Position files within their directories
    for (depth, dirs) in directories_by_depth.iter().enumerate() {
        let y = center.y - (max_depth as f32 * level_height) / 2.0 + depth as f32 * level_height;
        let total_width = dirs.len() as f32 * node_spacing;
        let start_x = center.x - total_width / 2.0 + node_spacing / 2.0;

        for (dir_index, dir) in dirs.iter().enumerate() {
            let dir_x = start_x + dir_index as f32 * node_spacing;
            
            if let Some(files_in_dir) = directory_tree.get(dir) {
                // Position files in this directory
                let files_per_row = 3;
                let file_spacing = 80.0 * zoom;
                
                for (file_index, file) in files_in_dir.iter().enumerate() {
                    let row = file_index / files_per_row;
                    let col = file_index % files_per_row;
                    
                    let files_in_row = (files_in_dir.len() - row * files_per_row).min(files_per_row);
                    let row_width = files_in_row as f32 * file_spacing;
                    let row_start_x = dir_x - row_width / 2.0 + file_spacing / 2.0;
                    
                    let file_x = row_start_x + col as f32 * file_spacing;
                    let file_y = y + 60.0 * zoom + row as f32 * 50.0 * zoom;
                    
                    file_positions.insert(file.clone(), egui::pos2(file_x, file_y));
                }
            }
        }
    }

    // Position elements around their files with better organization
    for element in &project.elements {
        if let Some(file_pos) = file_positions.get(&element.file_path) {
            let elements_in_file: Vec<_> = project.elements
                .iter()
                .filter(|e| e.file_path == element.file_path)
                .collect();
            
            let _element_index = elements_in_file
                .iter()
                .position(|e| e.id == element.id)
                .unwrap_or(0);

            // Group elements by type for better organization
            let element_type_index = match element.element_type {
                crate::parser::ElementType::Module => 0,
                crate::parser::ElementType::Struct => 1,
                crate::parser::ElementType::Enum => 2,
                crate::parser::ElementType::Trait => 3,
                crate::parser::ElementType::Impl => 4,
                crate::parser::ElementType::Function => 5,
            };

            // Create organized clusters around the file
            let cluster_angle = element_type_index as f32 * std::f32::consts::PI / 3.0;
            let cluster_radius = 45.0 * zoom;
            let cluster_center = *file_pos + egui::vec2(
                cluster_radius * cluster_angle.cos(),
                cluster_radius * cluster_angle.sin()
            );

            // Position within cluster
            let elements_of_same_type = elements_in_file
                .iter()
                .filter(|e| std::mem::discriminant(&e.element_type) == std::mem::discriminant(&element.element_type))
                .count();
            
            let type_element_index = elements_in_file
                .iter()
                .filter(|e| std::mem::discriminant(&e.element_type) == std::mem::discriminant(&element.element_type))
                .position(|e| e.id == element.id)
                .unwrap_or(0);

            if elements_of_same_type > 1 {
                let sub_angle = (type_element_index as f32 / elements_of_same_type as f32) * std::f32::consts::PI * 0.5;
                let sub_radius = 20.0 * zoom;
                let final_pos = cluster_center + egui::vec2(
                    sub_radius * sub_angle.cos(),
                    sub_radius * sub_angle.sin()
                );
                element_positions.insert(element.id.clone(), final_pos);
            } else {
                element_positions.insert(element.id.clone(), cluster_center);
            }
        }
    }
    
    (file_positions, element_positions)
}
