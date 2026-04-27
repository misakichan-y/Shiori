use crate::engine::renderer::render_object::RenderObject;

pub struct Scene {
    pub objects: Vec<RenderObject>,
}   

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }
    pub fn add_object(&mut self, object: RenderObject) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn extract_render_data(&self) -> &[RenderObject] {
        &self.objects
    }
}