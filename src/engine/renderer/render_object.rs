#[derive(Debug, Clone)]

pub struct RenderObject {
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub texture_id: usize,
    pub layer: i32,// 🔥 for future layering
}
