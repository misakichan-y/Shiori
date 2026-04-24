use ash::vk;
use crate::engine::renderer::device::Device;

pub struct VertexBuffer {
    pub buffer: vk::Buffer,
    pub memory: vk::DeviceMemory,
}

impl VertexBuffer {
    pub fn new(device: &Device) -> Self {
        let vertices: [[f32; 2]; 3] = [
            [0.0, -0.5],
            [0.5, 0.5],
            [-0.5, 0.5],
        ];

        let size = (vertices.len() * std::mem::size_of::<[f32; 2]>()) as u64;

        let buffer_info = vk::BufferCreateInfo {
            size,
            usage: vk::BufferUsageFlags::VERTEX_BUFFER,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };

        let buffer = unsafe {
            device.device.create_buffer(&buffer_info, None).unwrap()
        };

        let mem_requirements = unsafe {
            device.device.get_buffer_memory_requirements(buffer)
        };

        let mem_properties = unsafe {
            device.instance.get_physical_device_memory_properties(
                device.physical_device,
            )
        };

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

        let memory_type_index = memory_type_index.unwrap();

        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index,
            ..Default::default()
        };

        let memory = unsafe {
            device.device.allocate_memory(&alloc_info, None).unwrap()
        };

        unsafe {
            device.device.bind_buffer_memory(buffer, memory, 0).unwrap();
        }

        // 🔥 COPY DATA
        unsafe {
            let data_ptr = device.device.map_memory(
                memory,
                0,
                size,
                vk::MemoryMapFlags::empty(),
            ).unwrap();

            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                data_ptr as *mut u8,
                size as usize,
            );

            device.device.unmap_memory(memory);
        }

        println!("Vertex buffer created!");

        Self { buffer, memory }
    }
}