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

    // 🔥 EASING FUNCTION (move OUTSIDE update)
    fn ease_in_out(t: f32) -> f32 {
        t * t * (3.0 - 2.0 * t) // smoothstep
    }

    pub fn update(&mut self, delta: f32) {
        for obj in &mut self.objects {
            obj.time += delta;

            let mut t = obj.time / obj.duration;

            if t > 1.0 {
                t = 1.0;
            }

            let t = Self::ease_in_out(t);

            obj.position[0] =
                obj.start_position[0] +
                (obj.target_position[0] - obj.start_position[0]) * t;

            obj.position[1] =
                obj.start_position[1] +
                (obj.target_position[1] - obj.start_position[1]) * t;
        }
    }

    pub fn extract_render_data(&self) -> Vec<RenderObject> {
        let mut objects = self.objects.clone();
        objects.sort_by_key(|obj| obj.layer);
        objects
    }
}