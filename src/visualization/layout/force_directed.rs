use eframe::egui;
use std::collections::HashMap;
use crate::{project::Project, LayoutSettings};

pub struct ForceDirectedLayout {
    pub positions: HashMap<String, egui::Pos2>,
    pub velocities: HashMap<String, egui::Vec2>,
    pub forces: HashMap<String, egui::Vec2>,
    pub settings: LayoutSettings,
    pub is_stable: bool,
    pub iteration_count: u32,
    pub max_iterations: u32,
}

impl Default for ForceDirectedLayout {
    fn default() -> Self {
        Self {
            positions: HashMap::new(),
            velocities: HashMap::new(),
            forces: HashMap::new(),
            settings: LayoutSettings::default(),
            is_stable: false,
            iteration_count: 0,
            max_iterations: 1000,
        }
    }
}

impl ForceDirectedLayout {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.positions.clear();
        self.velocities.clear();
        self.forces.clear();
        self.iteration_count = 0;
        self.is_stable = false;
    }

    pub fn initialize_positions(&mut self, elements: &[crate::project::Element], center: egui::Pos2, bounds: egui::Rect) {
        self.positions.clear();
        self.velocities.clear();
        self.forces.clear();
        
        // Initialize positions in a circle or grid pattern
        let radius = bounds.width().min(bounds.height()) * 0.3;
        let count = elements.len();
        
        for (i, element) in elements.iter().enumerate() {
            let angle = if count > 1 {
                2.0 * std::f32::consts::PI * i as f32 / count as f32
            } else {
                0.0
            };
            
            let offset = egui::vec2(
                radius * angle.cos(),
                radius * angle.sin()
            );
            
            self.positions.insert(element.id.clone(), center + offset);
            self.velocities.insert(element.id.clone(), egui::Vec2::ZERO);
            self.forces.insert(element.id.clone(), egui::Vec2::ZERO);
        }
        
        self.iteration_count = 0;
        self.is_stable = false;
    }

    #[allow(dead_code)]
    pub fn update_settings(&mut self, layout_settings: &LayoutSettings) {
        self.settings = layout_settings.clone();
        // Reset stability when settings change
        self.is_stable = false;
        self.iteration_count = 0;
    }

    pub fn step(&mut self, project: &Project, dt: f32) -> bool {
        if self.is_stable || self.iteration_count >= self.max_iterations {
            return true;
        }

        // Clear forces
        for force in self.forces.values_mut() {
            *force = egui::Vec2::ZERO;
        }

        // Calculate repulsive forces between all nodes
        self.calculate_repulsive_forces(&project.elements);
        
        // Calculate attractive forces from relationships
        self.calculate_attractive_forces(project);
        
        // Apply forces and update positions
        let mut max_displacement: f32 = 0.0;
        
        for element in &project.elements {
            if let (Some(force), Some(velocity), Some(position)) = (
                self.forces.get(&element.id),
                self.velocities.get_mut(&element.id),
                self.positions.get_mut(&element.id)
            ) {
                // Update velocity with force and damping
                *velocity = *velocity * self.settings.damping + *force * dt;
                
                // Limit velocity
                let max_velocity = 50.0;
                if velocity.length() > max_velocity {
                    *velocity = velocity.normalized() * max_velocity;
                }
                
                // Update position
                let displacement = *velocity * dt;
                *position += displacement;
                
                max_displacement = max_displacement.max(displacement.length());
            }
        }

        self.iteration_count += 1;
        
        // Check for stability
        if max_displacement < 1.0 {
            self.is_stable = true;
        }

        self.is_stable
    }

    fn calculate_repulsive_forces(&mut self, elements: &[crate::project::Element]) {
        for i in 0..elements.len() {
            for j in (i + 1)..elements.len() {
                let element_a = &elements[i];
                let element_b = &elements[j];
                
                if let (Some(pos_a), Some(pos_b)) = (
                    self.positions.get(&element_a.id),
                    self.positions.get(&element_b.id)
                ) {
                    let diff = *pos_a - *pos_b;
                    let distance = diff.length().max(1.0); // Prevent division by zero
                    
                    // Coulomb's law: F = k * q1 * q2 / r^2
                    let force_magnitude = self.settings.repulsion_strength / (distance * distance);
                    let force_direction = diff.normalized();
                    let force = force_direction * force_magnitude;
                    
                    // Apply equal and opposite forces
                    if let Some(force_a) = self.forces.get_mut(&element_a.id) {
                        *force_a += force;
                    }
                    if let Some(force_b) = self.forces.get_mut(&element_b.id) {
                        *force_b -= force;
                    }
                }
            }
        }
    }

    fn calculate_attractive_forces(&mut self, project: &Project) {
        for relationship in &project.relationships {
            if let (Some(source_pos), Some(target_pos)) = (
                self.positions.get(&relationship.source_id),
                self.positions.get(&relationship.target_id)
            ) {
                let diff = *target_pos - *source_pos;
                let distance = diff.length();
                
                if distance > 0.0 {
                    // Hooke's law: F = k * (distance - rest_length)
                    let displacement = distance - self.settings.spring_length;
                    let force_magnitude = self.settings.force_strength * displacement;
                    let force_direction = diff.normalized();
                    let force = force_direction * force_magnitude;
                    
                    // Apply attractive force
                    if let Some(source_force) = self.forces.get_mut(&relationship.source_id) {
                        *source_force += force;
                    }
                    if let Some(target_force) = self.forces.get_mut(&relationship.target_id) {
                        *target_force -= force;
                    }
                }
            }
        }
    }

    pub fn get_positions(&self) -> &HashMap<String, egui::Pos2> {
        &self.positions
    }

    #[allow(dead_code)]
    pub fn is_complete(&self) -> bool {
        self.is_stable
    }
}
