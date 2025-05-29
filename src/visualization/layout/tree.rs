use eframe::egui;
use std::collections::{HashMap, HashSet};

use crate::project::Project;
use super::{LayoutNode, LayoutEdge, EdgeType};

pub struct TreeLayout {
    #[allow(dead_code)] // Fields for future tree layout customization
    pub level_height: f32,
    #[allow(dead_code)]
    pub node_spacing: f32,
}

impl Default for TreeLayout {
    fn default() -> Self {
        Self {
            level_height: 120.0,
            node_spacing: 150.0,
        }
    }
}

impl TreeLayout {
    #[allow(dead_code)] // Constructor for future use
    pub fn new() -> Self {
        Self::default()
    }
    
    #[allow(dead_code)] // Method for future tree layout implementation
    pub fn calculate(
        &mut self,
        nodes: &[LayoutNode],
        edges: &[LayoutEdge],
        center: egui::Pos2,
        zoom: f32,
    ) -> HashMap<String, egui::Pos2> {
        if nodes.is_empty() {
            return HashMap::new();
        }
        
        // Build adjacency list
        let mut children: HashMap<String, Vec<String>> = HashMap::new();
        let mut parents: HashMap<String, String> = HashMap::new();
        
        for edge in edges {
            if edge.edge_type == EdgeType::Contains {
                children.entry(edge.target.clone()).or_default().push(edge.source.clone());
                parents.insert(edge.source.clone(), edge.target.clone());
            }
        }
        
        // Find root nodes (nodes with no parents)
        let mut roots = Vec::new();
        for node in nodes {
            if !parents.contains_key(&node.id) {
                roots.push(node.id.clone());
            }
        }
        
        if roots.is_empty() {
            // Fallback to first node as root
            roots.push(nodes[0].id.clone());
        }
        
        let mut positions = HashMap::new();
        let mut used_x_positions = HashSet::new();
        
        // Layout each tree
        for (tree_index, root_id) in roots.iter().enumerate() {
            let tree_offset = tree_index as f32 * 400.0 * zoom;
            let tree_center = egui::pos2(center.x + tree_offset, center.y);
            
            self.layout_tree(
                root_id,
                &children,
                nodes,
                tree_center,
                0,
                0.0,
                zoom,
                &mut positions,
                &mut used_x_positions,
            );
        }
        
        positions
    }
    
    #[allow(dead_code)] // Helper method for tree layout
    fn layout_tree(
        &self,
        node_id: &str,
        children: &HashMap<String, Vec<String>>,
        nodes: &[LayoutNode],
        center: egui::Pos2,
        level: usize,
        x_offset: f32,
        zoom: f32,
        positions: &mut HashMap<String, egui::Pos2>,
        used_x_positions: &mut HashSet<i32>,
    ) -> f32 {
        let y = center.y + (level as f32 * self.level_height * zoom);
        
        if let Some(node_children) = children.get(node_id) {
            if node_children.is_empty() {
                // Leaf node
                let mut x = center.x + x_offset * zoom;
                
                // Ensure no overlap
                let x_key = (x / (self.node_spacing * zoom * 0.5)) as i32;
                while used_x_positions.contains(&x_key) {
                    x += self.node_spacing * zoom * 0.1;
                }
                used_x_positions.insert(x_key);
                
                positions.insert(node_id.to_string(), egui::pos2(x, y));
                return x;
            } else {
                // Internal node - layout children first
                let mut child_positions = Vec::new();
                let mut current_x_offset = x_offset - (node_children.len() as f32 * self.node_spacing * 0.5);
                
                for child_id in node_children {
                    let child_x = self.layout_tree(
                        child_id,
                        children,
                        nodes,
                        center,
                        level + 1,
                        current_x_offset,
                        zoom,
                        positions,
                        used_x_positions,
                    );
                    child_positions.push(child_x);
                    current_x_offset += self.node_spacing;
                }
                
                // Position parent at center of children
                let avg_x = if !child_positions.is_empty() {
                    child_positions.iter().sum::<f32>() / child_positions.len() as f32
                } else {
                    center.x + x_offset * zoom
                };
                
                positions.insert(node_id.to_string(), egui::pos2(avg_x, y));
                return avg_x;
            }
        } else {
            // No children
            let mut x = center.x + x_offset * zoom;
            
            // Ensure no overlap
            let x_key = (x / (self.node_spacing * zoom * 0.5)) as i32;
            while used_x_positions.contains(&x_key) {
                x += self.node_spacing * zoom * 0.1;
            }
            used_x_positions.insert(x_key);
            
            positions.insert(node_id.to_string(), egui::pos2(x, y));
            return x;
        }
    }
}

pub fn calculate(
    project: &Project,
    zoom: f32,
    center: egui::Pos2,
    _animation_progress: f32
) -> (HashMap<String, egui::Pos2>, HashMap<String, egui::Pos2>) {
    let mut file_positions = HashMap::new();
    let mut element_positions = HashMap::new();
    
    // Position files in a hierarchical tree
    let max_depth = 3;
    let width = 600.0 * zoom;
    let height = 500.0 * zoom;
    
    // Root at the top
    if let Some(first_file) = project.files.first() {
        file_positions.insert(
            first_file.clone(),
            center + egui::vec2(0.0, -height/2.0)
        );
    }
    
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
            
            // Arrange elements in a row below their file
            let total_width = 150.0 * zoom * elements_in_file as f32;
            let element_spacing = total_width / elements_in_file.max(1) as f32;
            let x_offset = -total_width / 2.0 + element_spacing / 2.0 + element_index as f32 * element_spacing;
            let y_offset = 60.0 * zoom;
            
            element_positions.insert(
                element.id.clone(), 
                *file_pos + egui::vec2(x_offset, y_offset)
            );
        }
    }
    
    (file_positions, element_positions)
}
