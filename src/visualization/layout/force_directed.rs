use eframe::egui;
use std::collections::HashMap;
use crate::{LayoutSettings, project::Project};
use rand::Rng;

pub struct ForceDirectedLayout {
    pub settings: ForceSettings,
}

#[derive(Debug, Clone)]
pub struct ForceSettings {
    pub attraction_strength: f32,
    pub repulsion_strength: f32,
    pub spring_length: f32,
    pub damping: f32,
    pub iterations: usize,
}

impl Default for ForceSettings {
    fn default() -> Self {
        Self {
            attraction_strength: 0.1,
            repulsion_strength: 100.0,
            spring_length: 50.0,
            damping: 0.9,
            iterations: 10,
        }
    }
}

impl ForceDirectedLayout {
    pub fn new() -> Self {
        Self {
            settings: ForceSettings::default(),
        }
    }
    
    #[allow(dead_code)] // Method for future layout customization
    pub fn update_settings(&mut self, layout_settings: &LayoutSettings) {
        self.settings.attraction_strength = layout_settings.force_strength;
        self.settings.repulsion_strength = layout_settings.repulsion_strength;
        self.settings.spring_length = layout_settings.spring_length;
        self.settings.damping = layout_settings.damping;
    }
    
    pub fn calculate_positions(
        &self,
        project: &Project,
        file_positions: &mut HashMap<String, egui::Pos2>,
        element_positions: &mut HashMap<String, egui::Pos2>,
        zoom: f32,
        center: egui::Pos2,
    ) {
        // Initialize positions randomly
        let mut rng = rand::thread_rng();
        
        // Position files first
        for file in &project.files {
            let x = center.x + (rng.gen::<f32>() - 0.5) * 400.0 * zoom;
            let y = center.y + (rng.gen::<f32>() - 0.5) * 300.0 * zoom;
            file_positions.insert(file.clone(), egui::pos2(x, y));
        }
        
        // Position elements around their files
        for element in &project.elements {
            if let Some(file_pos) = file_positions.get(&element.file_path) {
                let offset_x = (rng.gen::<f32>() - 0.5) * 80.0 * zoom;
                let offset_y = (rng.gen::<f32>() - 0.5) * 80.0 * zoom;
                element_positions.insert(
                    element.id.clone(),
                    *file_pos + egui::vec2(offset_x, offset_y)
                );
            } else {
                // Fallback position if file not found
                let x = center.x + (rng.gen::<f32>() - 0.5) * 400.0 * zoom;
                let y = center.y + (rng.gen::<f32>() - 0.5) * 300.0 * zoom;
                element_positions.insert(element.id.clone(), egui::pos2(x, y));
            }
        }
        
        // Apply force-directed algorithm for elements
        for _iteration in 0..self.settings.iterations {
            let mut forces: HashMap<String, egui::Vec2> = HashMap::new();
            
            // Calculate repulsion forces between all elements
            for element1 in &project.elements {
                let mut total_force = egui::Vec2::ZERO;
                
                if let Some(pos1) = element_positions.get(&element1.id) {
                    for element2 in &project.elements {
                        if element1.id != element2.id {
                            if let Some(pos2) = element_positions.get(&element2.id) {
                                let diff = *pos1 - *pos2;
                                let dist = diff.length().max(1.0);
                                let force = diff.normalized() * (self.settings.repulsion_strength / (dist * dist));
                                total_force += force;
                            }
                        }
                    }
                    
                    // Attraction to file center
                    if let Some(file_pos) = file_positions.get(&element1.file_path) {
                        let diff = *file_pos - *pos1;
                        let dist = diff.length();
                        if dist > self.settings.spring_length {
                            let force = diff.normalized() * self.settings.attraction_strength * (dist - self.settings.spring_length);
                            total_force += force;
                        }
                    }
                }
                
                forces.insert(element1.id.clone(), total_force);
            }
            
            // Apply forces
            for (element_id, force) in forces {
                if let Some(pos) = element_positions.get_mut(&element_id) {
                    let velocity = force * self.settings.damping;
                    *pos += velocity;
                }
            }
        }
        
        // Calculate file positions based on their elements
        for file in &project.files {
            let elements_in_file: Vec<_> = project.elements
                .iter()
                .filter(|e| e.file_path == *file)
                .collect();
            
            if !elements_in_file.is_empty() {
                let mut centroid = egui::Vec2::ZERO;
                let mut count = 0;
                
                for element in elements_in_file {
                    if let Some(pos) = element_positions.get(&element.id) {
                        centroid += pos.to_vec2();
                        count += 1;
                    }
                }
                
                if count > 0 {
                    centroid = centroid / count as f32;
                    file_positions.insert(file.clone(), egui::pos2(centroid.x, centroid.y));
                }
            }
        }
    }
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self::new()
    }
}
