pub mod instance;
pub mod surface;
pub mod device;
pub mod swapchain;
pub mod render_pass;
pub mod framebuffer;
pub mod pipeline;


use crate::engine::renderer::instance::VulkanInstance;
use crate::engine::renderer::surface::Surface;
use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::render_pass::RenderPass;
use crate::engine::renderer::framebuffer::Framebuffers;
use crate::engine::renderer::pipeline::Pipeline;


use winit::window::Window;

pub struct Renderer {
    pub instance: VulkanInstance,
    pub surface: Option<Surface>,
    pub device: Option<Device>,
    pub swapchain: Option<Swapchain>,
    pub render_pass: Option<RenderPass>,
    pub framebuffers: Option<Framebuffers>,
    pub pipeline: Option<Pipeline>,
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
            render_pass: None,
            framebuffers: None,
            pipeline: None,
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

    pub fn create_render_pass(&mut self) {
        let device =  self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();

        let render_pass = RenderPass::new(device, swapchain);

        self.render_pass = Some(render_pass);

    }
    
    pub fn create_framebuffers(&mut self) { 
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let render_pass = self.render_pass.as_ref().unwrap();

        let framebuffer = Framebuffers::new(
            device,
            swapchain,
            render_pass,

        );

        self.framebuffers = Some(framebuffer);

    }
    pub fn create_pipeline(&mut self) {
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let render_pass = self.render_pass.as_ref().unwrap();

        let pipeline = Pipeline::new(device, render_pass, swapchain);

        self.pipeline = Some(pipeline);

                                                
    }
}