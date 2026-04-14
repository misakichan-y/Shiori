use ash::{vk, Entry, Instance};
use winit::window::Window;

use raw_window_handle::{HasDisplayHandle, HasWindowHandle};

pub struct Surface {
    pub surface: vk::SurfaceKHR,
    pub loader: ash::khr::surface::Instance,
}

impl Surface {
    pub fn new(
        entry: &Entry,
        instance: &Instance,
        window: &Window,
    ) -> Self {
        let display_handle = window.display_handle().unwrap();
        let window_handle = window.window_handle().unwrap();

        let surface = unsafe {
            ash_window::create_surface(
                entry,
                instance,
                display_handle.as_raw(),
                window_handle.as_raw(),
                None,
            )
            .expect("Failed to create surface")
        };

        let loader = ash::khr::surface::Instance::new(entry, instance);

        Self {
            surface,
            loader,
        }
    }
}
