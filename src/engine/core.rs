use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::engine::renderer::Renderer;
use crate::engine::scene::Scene;
use crate::engine::renderer::render_object::RenderObject;

// 🔥 ADD THESE (you forgot them)
use crate::engine::scene::transition::{Transition, TransitionMode};

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

        // 🔥 Objects
        scene.add_object(RenderObject {
            position: [0.0, 0.0],
            scale: [0.5, 0.5],
            texture_id: 0,
            layer: 1,

            start_position: [0.0, 0.0],
            target_position: [0.0, 0.0],
            speed: 0.5,

            time: 0.0,
            duration: 2.0,
        });

        scene.add_object(RenderObject {
            position: [0.0, 0.0],
            scale: [0.2, 0.2],
            texture_id: 1,
            layer: 0,

            start_position: [0.0, 0.0],
            target_position: [0.5, 0.5],
            speed: 0.5,

            time: 0.0,
            duration: 2.0,
        });

        // 🔥 Camera
        scene.camera.position[0] = 0.5;

        // 🔥 START TRANSITION
        scene.transition = Some(
            Transition::new(2.0, TransitionMode::FadeIn)
        );

        // 🔥 Time tracking
        let mut last_time = std::time::Instant::now();

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

                            // 🔥 NOTE: transition not yet passed to renderer
                            renderer.render(objects, &scene.camera, scene.transition.as_ref());
                        }

                        WindowEvent::Resized(size) => {
                            renderer.resize(size.width, size.height);
                        }

                        _ => {}
                    },

                    Event::AboutToWait => {
                        // 🔥 DELTA TIME
                        let now = std::time::Instant::now();
                        let delta = (now - last_time).as_secs_f32();
                        last_time = now;

                        // 🔥 THIS WAS MISSING → animation + transition
                        scene.update(delta);

                        // (optional camera motion)
                        // scene.camera.position[0] += 0.5 * delta;

                        window_clone.request_redraw();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}