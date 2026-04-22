use super::entity::Entity;
use crate::engine::renderer::render_object::RenderObject;

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

    pub fn add_entity(&mut self) -> &mut Entity {
        let entity = Entity::new(self.next_id);
        self.next_id += 1;

        self.entities.push(entity);
        self.entities.last_mut().unwrap()
    }

    // 🔥 Update logic (runs every frame)
    pub fn update(&mut self) {
        for entity in &mut self.entities {
            // 🔄 Rotate each entity
            entity.transform.rotation[0] += 0.05;

            // (optional) keep rotation in range
            if entity.transform.rotation[0] > std::f32::consts::TAU {
                entity.transform.rotation[0] = 0.0;
            }
        }
    }

    // 🔥 Extract data for renderer (CPU → GPU)
    pub fn extract_render_data(&self) -> Vec<RenderObject> {
        let mut objects = Vec::new();

        for entity in &self.entities {
            if entity.visible {
                objects.push(RenderObject {
                    position: [
                        entity.transform.position[0],
                        entity.transform.position[1],
                    ],
                    rotation: entity.transform.rotation[0],
                    scale: [
                        entity.transform.scale[0],
                        entity.transform.scale[1],
                    ],
                });
            }
        }

        objects
    }
}