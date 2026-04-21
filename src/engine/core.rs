use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::engine::renderer::Renderer;

pub struct Engine {
    pub event_loop: EventLoop<()>,
}

impl Engine {
    pub fn new() -> Self {
        let event_loop = EventLoop::new().unwrap();
        Self { event_loop }
    }

    pub fn run(self) {
        let mut renderer = Renderer::new();

        let window = WindowBuilder::new()
            .with_title("Shiori")
            .with_visible(true)
            .build(&self.event_loop)
            .unwrap();

       renderer.create_surface(&window);
       renderer.create_device();
       renderer.create_swapchain();
       renderer.create_render_pass();
       renderer.create_framebuffers();
       renderer.create_pipeline();


        let window = std::sync::Arc::new(window);
        let window_clone = window.clone();

        self.event_loop
            .run(move |event, elwt| {
                match event {
                    Event::WindowEvent { event, .. } => match event {
                        WindowEvent::CloseRequested => {
                            elwt.exit();
                        }

                        WindowEvent::RedrawRequested => {
                           println!("Drawing frame...");
                        }

                        _ => {}
                    },

                    // This keeps the window updating
                    Event::AboutToWait => {
                        window_clone.request_redraw();
                    }

                    _ => {}
                }
            })
            .unwrap();
    }
}
