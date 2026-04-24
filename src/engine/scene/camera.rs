pub struct Camera {
    pub position: [f32; 2],
    pub zoom: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            position: [0.0, 0.0],
            zoom: 1.0,
        }
    }

   pub fn get_matrix(&self) -> [f32; 16] {
    let z = self.zoom;

    [
        z,   0.0, 0.0, 0.0,
        0.0, z,   0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        -self.position[0], -self.position[1], 0.0, 1.0,
    ]
  }
}
