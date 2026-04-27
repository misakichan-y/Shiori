use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::engine::renderer::Renderer;
use crate::engine::scene::Scene;
use crate::engine::renderer::render_object::RenderObject;

pub struct Engine {
    pub event_loop: EventLoop<()>,
}

impl Engine {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        Self { event_loop }
    }

    pub fn run(self) {
        // 🔥 Renderer
        let mut renderer = Renderer::new();

        // 🔥 Window
        let window = WindowBuilder::new()
            .with_title("Shiori")
            .build(&self.event_loop)
            .unwrap();

        // 🔥 Init renderer
        renderer.init(&window);

        // 🔥 Scene
        let mut scene = Scene::new();

        // Add some test objects
        scene.add_object(RenderObject {
            position: [0.0, 0.0],
            scale: [0.5, 0.5],
            texture_id: 0,
            layer: 1,
        });


        scene.add_object(RenderObject {
            position: [0.0, 0.0],
            scale: [0.2, 0.2],
            texture_id: 1,
            layer: 0
        });

        scene.camera.position[0] = -0.5;

        // Arc for winit
        let window = std::sync::Arc::new(window);
        let window_clone = window.clone();

        // 🔥 Main loop
        self.event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        WindowEvent::RedrawRequested => {
                            let objects = scene.extract_render_data();
                            renderer.render(objects, &scene.camera);
                        }

                        WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);
                        }

                        _ => {}
                    },

                    Event::AboutToWait => {
                        // (future: scene.update())
                        window_clone.request_redraw();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}