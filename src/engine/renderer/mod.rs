pub mod instance;
pub mod surface;
pub mod device;
pub mod swapchain;

use crate::engine::renderer::instance::VulkanInstance;
use crate::engine::renderer::surface::Surface;
use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;

use winit::window::Window;

pub struct Renderer {
    pub instance: VulkanInstance,
    pub surface: Option<Surface>,
    pub device: Option<Device>,
    pub swapchain: Option<Swapchain>,
}

impl Renderer {
    pub fn new() -> Self {
        let instance = VulkanInstance::new();
        println!("Vulkan Baby Started");

        Self {
            instance,
            surface: None,
            device: None,
            swapchain: None,
        }
    }

    // 🔥 Create Surface
    pub fn create_surface(&mut self, window: &Window) {
        let surface = Surface::new(
            &self.instance.entry,
            &self.instance.instance,
            window,
        );

        println!("Surface Works");

        self.surface = Some(surface);
    }

    // 🔥 Create Device (FIXED)
    pub fn create_device(&mut self) {
        let surface = self.surface.as_ref().unwrap();

        let device = Device::new(
            &self.instance.instance,
            &surface.loader,
            surface.surface,
        );

        println!("Device created! whooohoooo");

        self.device = Some(device);
    }

    // 🔥 Create Swapchain
    pub fn create_swapchain(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();

        let swapchain = Swapchain::new(
            &self.instance.instance,
            device,
            surface,
        );

        println!("Swapchain created!");

        self.swapchain = Some(swapchain);
    }
}