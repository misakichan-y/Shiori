use super::entity::Entity;

pub struct Scene {
    pub entities: Vec<Entity>,
    next_id: u32,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            next_id: 0,
        }
    }

    pub fn add_entity(&mut self) -> &Entity {
        let entity = Entity::new(self.next_id);
        self.next_id += 1;
        self.entities.push(entity);
        self.entities.last().unwrap()
    }
    pub fn update(&mut self) {
        // Update logic for the scene, e.g., animations, physics, etc.
        for entity in &mut self.entities {
            // Update each entity's transform or other properties as needed
            let _ = entity;
        }
    }
}