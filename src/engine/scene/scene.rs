use crate::engine::renderer::render_object::RenderObject;
use crate::engine::scene::camera::Camera;
pub struct Scene {
    pub objects: Vec<RenderObject>,
    pub camera: Camera,
}   

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            camera: Camera::new(),
        }
    }
    pub fn add_object(&mut self, object: RenderObject) {
        self.objects.push(object);
    }
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn extract_render_data(&self) -> Vec<RenderObject> {
        let mut objects = self.objects.clone();

        objects.sort_by_key(|obj| obj.layer);

        objects
    }
}