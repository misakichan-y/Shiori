use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::framebuffer::Framebuffers;
use crate::engine::renderer::pipeline::Pipeline;
use crate::engine::renderer::render_pass::RenderPass;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>,
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl Commands {
    pub fn new(
        device: &Device,
        _swapchain: &crate::engine::renderer::swapchain::Swapchain,
        render_pass: &RenderPass,
        framebuffers: &Framebuffers,
        pipeline: &Pipeline,
    ) -> Self {
        let pool_info = vk::CommandPoolCreateInfo {
            queue_family_index: device.queue_family_index,
            ..Default::default()
        };

        let command_pool = unsafe {
            device.device
                .create_command_pool(&pool_info, None)
                .expect("Failed to create command pool")
        };

        let alloc_info = vk::CommandBufferAllocateInfo {
            command_pool,
            level: vk::CommandBufferLevel::PRIMARY,
            command_buffer_count: framebuffers.framebuffers.len() as u32,
            ..Default::default()
        };

        let buffers = unsafe {
            device.device
                .allocate_command_buffers(&alloc_info)
                .expect("Failed to allocate command buffers")
        };

        println!("Command buffers allocated!");

        Self {
            command_pool,
            buffers,
            layout: pipeline.layout,
            pipeline: pipeline.pipeline,
            render_pass: render_pass.render_pass,
            framebuffers: framebuffers.framebuffers.clone(),
        }
    }
}