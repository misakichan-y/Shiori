use ash::vk;

use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::command::Commands;

pub struct Draw {
    pub image_available: vk::Semaphore,
    pub render_finished: vk::Semaphore,
}

impl Draw {
    pub fn new(device: &Device) -> Self {
        let semaphore_info = vk::SemaphoreCreateInfo::default();

        let image_available = unsafe {
            device.device
                .create_semaphore(&semaphore_info, None)
                .unwrap()
        };

        let render_finished = unsafe {
            device.device
                .create_semaphore(&semaphore_info, None)
                .unwrap()
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
    ) {
        let (image_index, _) = unsafe {
            swapchain.loader.acquire_next_image(
                swapchain.swapchain,
                u64::MAX,
                self.image_available,
                vk::Fence::null(),
            ).unwrap()
        };

        let wait_semaphores = [self.image_available];
        let signal_semaphores = [self.render_finished];

        let submit_info = vk::SubmitInfo {
            wait_semaphore_count: 1,
            p_wait_semaphores: wait_semaphores.as_ptr(),

            p_wait_dst_stage_mask: [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT].as_ptr(),

            command_buffer_count: 1,
            p_command_buffers: &commands.buffers[image_index as usize],

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