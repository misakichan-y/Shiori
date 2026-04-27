use ash::vk;
use image::GenericImageView;

use crate::engine::renderer::device::Device;

pub struct Texture {
    pub image: vk::Image,
    pub memory: vk::DeviceMemory,
    pub view: vk::ImageView,
    pub sampler: vk::Sampler,
}

impl Texture {
    pub fn new(device: &Device, path: &str) -> Self {
        // 🔥 Load image
        let img = image::open(path).expect("Failed to load image");
        let (width, height) = img.dimensions();
        let pixels = img.to_rgba8();
        let image_size = (4 * width * height) as vk::DeviceSize;

        // 🔹 Create image (FIXED)
        let image_info = vk::ImageCreateInfo {
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::R8G8B8A8_UNORM,
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::LINEAR, // still simple mode
            usage: vk::ImageUsageFlags::SAMPLED,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            initial_layout: vk::ImageLayout::GENERAL, // 🔥 FIXED
            ..Default::default()
        };

        let image = unsafe {
            device.device.create_image(&image_info, None).unwrap()
        };

        // 🔹 Memory
        let mem_req = unsafe {
            device.device.get_image_memory_requirements(image)
        };

        let mem_props = unsafe {
            device.instance.get_physical_device_memory_properties(
                device.physical_device,
            )
        };

        let mut memory_type_index = None;

        for i in 0..mem_props.memory_type_count {
            let mem_type = mem_props.memory_types[i as usize];

            if (mem_req.memory_type_bits & (1 << i)) != 0
                && mem_type.property_flags.contains(
                    vk::MemoryPropertyFlags::HOST_VISIBLE
                        | vk::MemoryPropertyFlags::HOST_COHERENT,
                )
            {
                memory_type_index = Some(i);
                break;
            }
        }

        let memory_type_index = memory_type_index
            .expect("Failed to find memory type");

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_req.size,
            memory_type_index,
            ..Default::default()
        };

        let memory = unsafe {
            device.device.allocate_memory(&alloc_info, None).unwrap()
        };

        unsafe {
            device.device.bind_image_memory(image, memory, 0).unwrap();
        }

        // 🔥 COPY PIXELS
        unsafe {
            let data = device.device.map_memory(
                memory,
                0,
                image_size,
                vk::MemoryMapFlags::empty(),
            ).unwrap();

            std::ptr::copy_nonoverlapping(
                pixels.as_ptr(),
                data as *mut u8,
                image_size as usize,
            );

            device.device.unmap_memory(memory);
        }

        // 🔹 Image view
        let view_info = vk::ImageViewCreateInfo {
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: vk::Format::R8G8B8A8_UNORM,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            },
            ..Default::default()
        };

        let view = unsafe {
            device.device.create_image_view(&view_info, None).unwrap()
        };

        // 🔹 Sampler (IMPROVED)
        let sampler_info = vk::SamplerCreateInfo {
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            address_mode_u: vk::SamplerAddressMode::REPEAT,
            address_mode_v: vk::SamplerAddressMode::REPEAT,
            address_mode_w: vk::SamplerAddressMode::REPEAT,
            anisotropy_enable: vk::TRUE,
            max_anisotropy: 16.0,
            border_color: vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates: vk::FALSE,
            compare_enable: vk::FALSE,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            ..Default::default()
        };

        let sampler = unsafe {
            device.device.create_sampler(&sampler_info, None).unwrap()
        };

        println!("Texture loaded: {}", path);

        Self {
            image,
            memory,
            view,
            sampler,
        }
    }
}