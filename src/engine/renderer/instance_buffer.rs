use ash::vk;
use crate::engine::renderer::device::Device;

pub struct InstanceBuffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

impl InstanceBuffer {
    pub fn new(device: &Device, count: usize) -> Self {
        // 🔹 Create buffer
        let buffer_info = vk::BufferCreateInfo {
            size: (count * std::mem::size_of::<[f32; 5]>()) as u64,
            usage: vk::BufferUsageFlags::VERTEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let buffer = unsafe {
            device.device
                .create_buffer(&buffer_info, None)
                .expect("Failed to create instance buffer")
        };

        // 🔹 Get memory requirements
        let mem_requirements = unsafe {
            device.device.get_buffer_memory_requirements(buffer)
        };

        // 🔥 Get physical memory properties
        let mem_properties = unsafe {
            device.instance.get_physical_device_memory_properties(
                device.physical_device,
            )
        };

        // 🔥 Find suitable memory type
        let mut memory_type_index = None;

        for i in 0..mem_properties.memory_type_count {
            let mem_type = mem_properties.memory_types[i as usize];

            if (mem_requirements.memory_type_bits & (1 << i)) != 0
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
            .expect("Failed to find suitable memory type");

        // 🔹 Allocate memory
        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index,
            ..Default::default()
        };

        let memory = unsafe {
            device.device
                .allocate_memory(&alloc_info, None)
                .expect("Failed to allocate instance buffer memory")
        };

        // 🔹 Bind memory to buffer
        unsafe {
            device.device
                .bind_buffer_memory(buffer, memory, 0)
                .expect("Failed to bind buffer memory");
        }

        println!("Instance buffer created!");

        Self {
            buffer,
            memory,
        }
    }
}