use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::engine::renderer::Renderer;
use crate::engine::scene::Scene;

pub struct Engine {
    pub event_loop: EventLoop<()>,
}

impl Engine {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        Self { event_loop }
    }

    pub fn run(self) {
        // 🔥 Create renderer
        let mut renderer = Renderer::new();

        // 🔥 Create window
        let window = WindowBuilder::new()
            .with_title("Shiori")
            .build(&self.event_loop)
            .unwrap();

        // 🔥 Initialize renderer (single call)
        renderer.init(&window);

        // 🔥 Create scene
        let mut scene = Scene::new();
        scene.add_entity();

        // Arc for winit loop
        let window = std::sync::Arc::new(window);
        let window_clone = window.clone();

        // 🔥 Event loop
        self.event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        // 🔥 Render frame
                        WindowEvent::RedrawRequested => {
                            let objects = scene.extract_render_data();
                            renderer.render(&objects, &scene.camera);

                        }

                        // 🔥 Handle resize (stub for now)
                        WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);
                        }

                        _ => {}
                    },

                    // 🔥 Game loop tick
                    Event::AboutToWait => {
                        scene.update();
                        window_clone.request_redraw();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}