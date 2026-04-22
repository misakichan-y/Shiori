use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::command::Commands;
use crate::engine::renderer::render_object::RenderObject;
use crate::engine::renderer::instance_buffer::InstanceBuffer;
use crate::engine::scene::camera::Camera;

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
        objects: &[RenderObject],
        instance_buffer: &InstanceBuffer,
        camera: &Camera,
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
            // 🔹 Reset + begin command buffer
            device.device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();

            let begin_info = vk::CommandBufferBeginInfo::default();
            device.device.begin_command_buffer(cmd, &begin_info).unwrap();

            // 🔥 BEGIN RENDER PASS
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

            // 🔥 SEND CAMERA TO SHADER (CRITICAL)
            let cam = camera.get_matrix();

            device.device.cmd_push_constants(
                cmd,
                commands.layout,
                vk::ShaderStageFlags::VERTEX,
                0,
                std::slice::from_raw_parts(
                    cam.as_ptr() as *const u8,
                    std::mem::size_of::<[f32; 9]>(),
                ),
            );

            // 🔹 Viewport + Scissor
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

            // 🔥 ===== INSTANCING WITH TRANSFORMS =====

            let instance_data: Vec<[f32; 5]> = objects.iter().map(|o| {
                [
                    o.position[0],
                    o.position[1],
                    o.rotation,
                    o.scale[0],
                    o.scale[1],
                ]
            }).collect();
             
             // 🔥 DEBUG: how many instances will be drawn?
             println!("instances to draw: {}", instance_data.len());

            let size =
                (instance_data.len() * std::mem::size_of::<[f32; 5]>()) as u64;

            let data_ptr = device.device
                .map_memory(
                    instance_buffer.memory,
                    0,
                    size,
                    vk::MemoryMapFlags::empty(),
                )
                .unwrap();

            std::ptr::copy_nonoverlapping(
                instance_data.as_ptr() as *const u8,
                data_ptr as *mut u8,
                size as usize,
            );

            device.device.unmap_memory(instance_buffer.memory);

            // 🔹 Bind instance buffer
            device.device.cmd_bind_vertex_buffers(
                cmd,
                0,
                &[instance_buffer.buffer],
                &[0],
            );

            // 🔥 ONE INSTANCED DRAW CALL
            device.device.cmd_draw(
                cmd,
                3,
                instance_data.len() as u32,
                0,
                0,
            );

            // 🔥 ===== END =====

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
                .queue_submit(
                    device.graphics_queue,
                    &[submit_info],
                    vk::Fence::null(),
                )
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

            // ⚠️ temporary (slow but safe)
            device.device.queue_wait_idle(device.graphics_queue).unwrap();
        }
        
    }
}