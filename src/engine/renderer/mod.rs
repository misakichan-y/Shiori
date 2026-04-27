pub mod instance;
pub mod surface;
pub mod device;
pub mod swapchain;
pub mod render_pass;
pub mod framebuffer;
pub mod pipeline;
pub mod command;
pub mod draw;
pub mod render_object;
pub mod vertex_buffer;
pub mod descriptor;
pub mod texture;

use crate::engine::renderer::instance::VulkanInstance;
use crate::engine::renderer::surface::Surface;
use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::render_pass::RenderPass;
use crate::engine::renderer::framebuffer::Framebuffers;
use crate::engine::renderer::pipeline::Pipeline;
use crate::engine::renderer::command::Commands;
use crate::engine::renderer::draw::Draw;
use crate::engine::renderer::vertex_buffer::VertexBuffer;
use crate::engine::renderer::descriptor::Descriptor;
use crate::engine::renderer::texture::Texture;
use crate::engine::renderer::render_object::RenderObject;
use crate::engine::scene::camera::Camera;
use crate::engine::scene::transition::Transition;

use winit::window::Window;

pub struct Renderer {
    instance: VulkanInstance,
    surface: Option<Surface>,
    device: Option<Device>,
    swapchain: Option<Swapchain>,
    render_pass: Option<RenderPass>,
    framebuffers: Option<Framebuffers>,
    pipeline: Option<Pipeline>,
    commands: Option<Commands>,
    draw: Option<Draw>,
    vertex_buffer: Option<VertexBuffer>,

    // 🔥 MULTI TEXTURE
    textures: Vec<Texture>,
    descriptors: Vec<Descriptor>,
    transition: Option<Transition>,
}

impl Renderer {
    pub fn new() -> Self {
        let instance = VulkanInstance::new();
        println!("Vulkan Started");

        Self {
            instance,
            surface: None,
            device: None,
            swapchain: None,
            render_pass: None,
            framebuffers: None,
            pipeline: None,
            commands: None,
            draw: None,
            vertex_buffer: None,
            textures: Vec::new(),
            descriptors: Vec::new(),
            transition: None,
        }
    }

    pub fn init(&mut self, window: &Window) {
        self.create_surface(window);
        self.create_device();
        self.create_swapchain();
        self.create_render_pass();
        self.create_framebuffers();
        self.create_pipeline();

        // 🔥 IMPORTANT ORDER
        self.create_textures();
        self.create_descriptors();

        self.create_commands();
        self.create_draw();
        self.create_vertex_buffer();
    }

    fn create_surface(&mut self, window: &Window) {
        let surface = Surface::new(
            &self.instance.entry,
            &self.instance.instance,
            window,
        );

        println!("Surface OK");
        self.surface = Some(surface);
    }

    fn create_device(&mut self) {
        let surface = self.surface.as_ref().unwrap();

        let device = Device::new(
            &self.instance.instance,
            &surface.loader,
            surface.surface,
        );

        println!("Device OK");
        self.device = Some(device);
    }

    fn create_swapchain(&mut self) {
        let surface = self.surface.as_ref().unwrap();
        let device = self.device.as_ref().unwrap();

        let swapchain = Swapchain::new(
            &self.instance.instance,
            device,
            surface,
        );

        println!("Swapchain OK");
        self.swapchain = Some(swapchain);
    }

    fn create_render_pass(&mut self) {
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();

        let render_pass = RenderPass::new(device, swapchain);
        self.render_pass = Some(render_pass);
    }

    fn create_framebuffers(&mut self) {
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let render_pass = self.render_pass.as_ref().unwrap();

        let framebuffers = Framebuffers::new(
            device,
            swapchain,
            render_pass,
        );

        self.framebuffers = Some(framebuffers);
    }

    fn create_pipeline(&mut self) {
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let render_pass = self.render_pass.as_ref().unwrap();

        let pipeline = Pipeline::new(device, render_pass, swapchain);
        self.pipeline = Some(pipeline);
    }

    // 🔥 MULTIPLE TEXTURES
    fn create_textures(&mut self) {
        let device = self.device.as_ref().unwrap();

        let tex1 = Texture::new(device, "assets/texture1.png");
        let tex2 = Texture::new(device, "assets/texture2.png");

        self.textures = vec![tex1, tex2];
    }

    // 🔥 DESCRIPTORS FOR EACH TEXTURE
    fn create_descriptors(&mut self) {
        let device = self.device.as_ref().unwrap();
        let pipeline = self.pipeline.as_ref().unwrap();

        let mut descriptors = Vec::new();

        for tex in &self.textures {
            let desc = Descriptor::new(
                device,
                pipeline.descriptor_set_layout,
                tex,
            );
            descriptors.push(desc);
        }

        self.descriptors = descriptors;
    }

    fn create_commands(&mut self) {
        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let render_pass = self.render_pass.as_ref().unwrap();
        let framebuffers = self.framebuffers.as_ref().unwrap();
        let pipeline = self.pipeline.as_ref().unwrap();

        let commands = Commands::new(
            device,
            swapchain,
            render_pass,
            framebuffers,
            pipeline,
        );

        self.commands = Some(commands);
    }

    fn create_draw(&mut self) {
        let device = self.device.as_ref().unwrap();
        let draw = Draw::new(device);
        self.draw = Some(draw);
    }

    fn create_vertex_buffer(&mut self) {
        let device = self.device.as_ref().unwrap();
        let vb = VertexBuffer::new(device);
        self.vertex_buffer = Some(vb);
    }

    // 🔥 FINAL RENDER
    pub fn render(&self, objects: Vec<RenderObject>, camera: &Camera, transition: Option<&Transition>) {    
        if self.device.is_none() {
            return;
        }

        let device = self.device.as_ref().unwrap();
        let swapchain = self.swapchain.as_ref().unwrap();
        let commands = self.commands.as_ref().unwrap();
        let draw = self.draw.as_ref().unwrap();
        let vertex_buffer = self.vertex_buffer.as_ref().unwrap();

        draw.draw_frame(
            device,
            swapchain,
            commands,
            vertex_buffer,
            &self.descriptors,
            &objects,
            camera,
            transition,
        );
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        println!("Resize not implemented yet");
    }
}