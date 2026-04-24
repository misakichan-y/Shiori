use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::command::Commands;
use crate::engine::renderer::vertex_buffer::VertexBuffer; // 👈 IMPORTANT

pub struct Draw {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
}

impl Draw {
    pub fn new(device: &Device) -> Self {
        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let image_available = unsafe {
            device.device.create_semaphore(&semaphore_info, None).unwrap()
        };

        let render_finished = unsafe {
            device.device.create_semaphore(&semaphore_info, None).unwrap()
        };

        Self {
            image_available,
            render_finished,
        }
    }

    pub fn draw_frame(
        &self,
        device: &Device,
        swapchain: &Swapchain,
        commands: &Commands,
        vertex_buffer: &VertexBuffer, // 👈 ONLY THIS
    ) {
        // 🔹 Acquire image
        let (image_index, _) = unsafe {
            swapchain.loader.acquire_next_image(
                swapchain.swapchain,
                u64::MAX,
                self.image_available,
                vk::Fence::null(),
            ).unwrap()
        };

        let cmd = commands.buffers[image_index as usize];

        unsafe {
            // 🔹 Reset + begin
            device.device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();

            device.device.begin_command_buffer(
                cmd,
                &vk::CommandBufferBeginInfo::default(),
            ).unwrap();

            // 🔥 RENDER PASS
            let clear = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.1, 0.1, 0.1, 1.0],
                },
            };

            let render_pass_info = vk::RenderPassBeginInfo {
                render_pass: commands.render_pass,
                framebuffer: commands.framebuffers[image_index as usize],
                render_area: vk::Rect2D {
                    offset: vk::Offset2D { x: 0, y: 0 },
                    extent: swapchain.extent,
                },
                clear_value_count: 1,
                p_clear_values: &clear,
                ..Default::default()
            };

            device.device.cmd_begin_render_pass(
                cmd,
                &render_pass_info,
                vk::SubpassContents::INLINE,
            );

            // 🔹 Bind pipeline
            device.device.cmd_bind_pipeline(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                commands.pipeline,
            );

            // 🔹 Viewport
            let viewport = vk::Viewport {
                x: 0.0,
                y: 0.0,
                width: swapchain.extent.width as f32,
                height: swapchain.extent.height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            };

            let scissor = vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: swapchain.extent,
            };

            device.device.cmd_set_viewport(cmd, 0, &[viewport]);
            device.device.cmd_set_scissor(cmd, 0, &[scissor]);

            // 🔥 Bind vertex buffer (THIS IS KEY)
            device.device.cmd_bind_vertex_buffers(
                cmd,
                0,
                &[vertex_buffer.buffer],
                &[0],
            );

            // 🔥 DRAW TRIANGLE
            device.device.cmd_draw(
                cmd,
                3, // 3 vertices
                1, // 1 instance
                0,
                0,
            );

            // 🔹 End render pass
            device.device.cmd_end_render_pass(cmd);
            device.device.end_command_buffer(cmd).unwrap();
        }

        // 🔹 Submit
        let wait_semaphores = [self.image_available];
        let signal_semaphores = [self.render_finished];
        let wait_stages = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = vk::SubmitInfo {
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores.as_ptr(),
            p_wait_dst_stage_mask: wait_stages.as_ptr(),
            command_buffer_count: 1,
            p_command_buffers: &cmd,
            signal_semaphore_count: 1,
            p_signal_semaphores: signal_semaphores.as_ptr(),
            ..Default::default()
        };

        unsafe {
            device.device
                .queue_submit(device.graphics_queue, &[submit_info], vk::Fence::null())
                .unwrap();
        }

        // 🔹 Present
        let swapchains = [swapchain.swapchain];

        let present_info = vk::PresentInfoKHR {
            wait_semaphore_count: 1,
            p_wait_semaphores: signal_semaphores.as_ptr(),
            swapchain_count: 1,
            p_swapchains: swapchains.as_ptr(),
            p_image_indices: &image_index,
            ..Default::default()
        };

        unsafe {
            swapchain.loader
                .queue_present(device.graphics_queue, &present_info)
                .unwrap();

            device.device.queue_wait_idle(device.graphics_queue).unwrap();
        }
    }
}