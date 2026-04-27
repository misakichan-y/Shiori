use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::command::Commands;
use crate::engine::renderer::vertex_buffer::VertexBuffer;
use crate::engine::renderer::descriptor::Descriptor;
use crate::engine::renderer::render_object::RenderObject;
use crate::engine::scene::camera::Camera;
use crate::engine::scene::transition::Transition; // 🔥 NEW

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
        vertex_buffer: &VertexBuffer,
        descriptors: &[Descriptor],
        objects: &[RenderObject],
        camera: &Camera,
        transition: Option<&Transition>, // 🔥 NEW
    ) {
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
            device.device
                .reset_command_buffer(cmd, vk::CommandBufferResetFlags::empty())
                .unwrap();

            device.device.begin_command_buffer(
                cmd,
                &vk::CommandBufferBeginInfo::default(),
            ).unwrap();

            let clear = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.1, 0.0, 0.0, 1.0],
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

            device.device.cmd_bind_pipeline(
                cmd,
                vk::PipelineBindPoint::GRAPHICS,
                commands.pipeline,
            );

            device.device.cmd_bind_vertex_buffers(
                cmd,
                0,
                &[vertex_buffer.buffer],
                &[0],
            );

            // 🔥 DRAW OBJECTS
            for obj in objects {
                let descriptor = &descriptors[obj.texture_id];

                device.device.cmd_bind_descriptor_sets(
                    cmd,
                    vk::PipelineBindPoint::GRAPHICS,
                    commands.layout,
                    0,
                    &[descriptor.set],
                    &[],
                );

                // 🔥 FIXED CAMERA
                let world_x = (obj.position[0] - camera.position[0]) * camera.zoom;
                let world_y = (obj.position[1] - camera.position[1]) * camera.zoom;

                let data = [
                    world_x,
                    world_y,
                    obj.scale[0] * camera.zoom,
                    obj.scale[1] * camera.zoom,
                ];

                device.device.cmd_push_constants(
                    cmd,
                    commands.layout,
                    vk::ShaderStageFlags::VERTEX,
                    0,
                    std::slice::from_raw_parts(
                        data.as_ptr() as *const u8,
                        std::mem::size_of::<[f32; 4]>(),
                    ),
                );

                device.device.cmd_draw(cmd, 6, 1, 0, 0);
            }

            // 🔥 DRAW FADE OVERLAY
            if let Some(t) = transition {
                if t.active || t.alpha > 0.0 {
                    // 🔥 fullscreen quad
                    let data = [0.0, 0.0, 2.0, 2.0];

                    device.device.cmd_push_constants(
                        cmd,
                        commands.layout,
                        vk::ShaderStageFlags::VERTEX,
                        0,
                        std::slice::from_raw_parts(
                            data.as_ptr() as *const u8,
                            std::mem::size_of::<[f32; 4]>(),
                        ),
                    );

                    // 🔥 You will multiply fragment alpha by t.alpha in shader
                    device.device.cmd_draw(cmd, 6, 1, 0, 0);
                }
            }

            device.device.cmd_end_render_pass(cmd);
            device.device.end_command_buffer(cmd).unwrap();
        }

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