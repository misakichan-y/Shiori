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
           entity.transform.rotation[0] += 0.01; // Example: Rotate all entities

           if entity.transform.rotation[0] > 1.0 {
               entity.transform.rotation[0] = -1.0,
           }
        }
    }
}