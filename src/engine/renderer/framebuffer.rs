use ash::vk;
use crate::engine::renderer::device::Device;
use crate::engine::renderer::swapchain::Swapchain;
use crate::engine::renderer::render_pass::RenderPass;

pub struct Framebuffers {
    pub framebuffers: Vec<vk::Framebuffer>,
}

impl Framebuffers {
    pub fn new(
        device: &Device,
        swapchain: &Swapchain,
        render_pass: &RenderPass,
    ) -> Self {
        let mut framebuffers = vec![];
        
        for &image in &swapchain.images {
            let image_view_info = vk::ImageViewCreateInfo {
                image,
                view_type: vk::ImageViewType::TYPE_2D,
                format: swapchain.format,

                components: vk::ComponentMapping::default(),
                
                 subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,

                },

                ..Default::default()

            };
            let image_view = unsafe {
                device.device
                     .create_image_view(&image_view_info, None)
                     .expect("Failed to create image view")
            };
            let attachments = [image_view];

            let framebuffer_info = vk::FramebufferCreateInfo { 
                render_pass: render_pass.render_pass,
                attachment_count: 1,
                p_attachments: attachments.as_ptr(),
                width: swapchain.extent.width,
                height: swapchain.extent.height,
                layers: 1,
                ..Default::default()

            };
            
            let framebuffer = unsafe {
                device.device
                    .create_framebuffer(&framebuffer_info, None)
                    .expect("Failed to create framebuffer")
            };

            framebuffers.push(framebuffer);
        }
        println!("Framebuffers created: {}", framebuffers.len());

        Self { framebuffers }
        }
}