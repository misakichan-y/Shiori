use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::render_pass::RenderPass;
use crate::engine::renderer::framebuffer::Framebuffers;
use crate::engine::renderer::pipeline::Pipeline;

pub struct Commands {
    pub command_pool: vk::CommandPool,
    pub buffers: Vec<vk::CommandBuffer>,
}

impl Commands {
    pub fn new(
        device: &Device,
        swapchain: &Swapchain,
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

       
        for (i, &cmd) in buffers.iter().enumerate() {
            let begin_info = vk::CommandBufferBeginInfo::default();

            unsafe {
                device.device
                    .begin_command_buffer(cmd, &begin_info)
                    .unwrap();
            }

            let clear_values = [vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.1, 0.1, 0.1, 1.0],
                },
            }];

            let render_pass_info = vk::RenderPassBeginInfo {
                render_pass: render_pass.render_pass,
                framebuffer: framebuffers.framebuffers[i],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: swapchain.extent,
                },
                clear_value_count: clear_values.len() as u32,
                p_clear_values: clear_values.as_ptr(),
                ..Default::default()
            };

            unsafe {
                device.device.cmd_begin_render_pass(
                    cmd,
                    &render_pass_info,
                    vk::SubpassContents::INLINE,
                );

                device.device.cmd_bind_pipeline(
                    cmd,
                    vk::PipelineBindPoint::GRAPHICS,
                    pipeline.pipeline,
                );

                device.device.cmd_draw(cmd, 3, 1, 0, 0);

                device.device.cmd_end_render_pass(cmd);

                device.device.end_command_buffer(cmd).unwrap();
            }
        }

        println!("Command buffers recorded!");

        Self {
            command_pool,
            buffers,
        }
    }
}