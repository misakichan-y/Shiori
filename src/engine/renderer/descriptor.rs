use ash::vk;
use crate::engine::renderer::device::Device;
use crate::engine::renderer::texture::Texture;

pub struct Descriptor {
    pub pool: vk::DescriptorPool,
    pub set: vk::DescriptorSet,
}

impl Descriptor {
    pub fn new(
        device: &Device,
        layout: vk::DescriptorSetLayout,
        texture: &Texture,
    ) -> Self {
        let pool_size = vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: 1,
        };

        let pool_info = vk::DescriptorPoolCreateInfo {
            pool_size_count: 1,
            p_pool_sizes: &pool_size,
            max_sets: 1,
            ..Default::default()
        };

        let pool = unsafe {
            device.device.create_descriptor_pool(&pool_info, None).unwrap()
        };

        let alloc_info = vk::DescriptorSetAllocateInfo {
            descriptor_pool: pool,
            descriptor_set_count: 1,
            p_set_layouts: &layout,
            ..Default::default()
        };

        let set = unsafe {
            device.device.allocate_descriptor_sets(&alloc_info).unwrap()[0]
        };

        let image_info = vk::DescriptorImageInfo {
            image_layout: vk::ImageLayout::GENERAL,
            image_view: texture.view,
            sampler: texture.sampler,
        };

        let write = vk::WriteDescriptorSet {
            dst_set: set,
            dst_binding: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_image_info: &image_info,
            ..Default::default()
        };

        unsafe {
            device.device.update_descriptor_sets(&[write], &[]);
        }

        Self { pool, set }
    }
}