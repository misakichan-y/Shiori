use super::transform::Transform;

pub struct Entity {
    pub transform: Transform,
    pub id: u32,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self {
            transform: Transform::new(),
            id,
        }
    }
}