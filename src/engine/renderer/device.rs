use ash::{vk, Instance};

pub struct Device {
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub graphics_queue: vk::Queue,
    pub queue_family_index: u32,
}

impl Device {
    pub fn new(
        instance: &Instance,
        surface_loader: &ash::khr::surface::Instance,
        surface: vk::SurfaceKHR,
    ) -> Self {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .expect("Failed to get GPUs")
        };

        if physical_devices.is_empty() {
            panic!("No GPU found!");
        }

        println!("Found {} GPU(s)", physical_devices.len());

        let physical_device = physical_devices[0];

        println!("Using first GPU");

        let queue_families = unsafe {
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let mut queue_family_index = None;

        for (index, family) in queue_families.iter().enumerate() {
            let supports_graphics =
                family.queue_flags.contains(vk::QueueFlags::GRAPHICS);

            let supports_present = unsafe {
                surface_loader
                    .get_physical_device_surface_support(
                        physical_device,
                        index as u32,
                        surface,
                    )
                    .unwrap()
            };

            if supports_graphics && supports_present {
                queue_family_index = Some(index as u32);
                break;
            }
        }

        let queue_family_index =
            queue_family_index.expect("No suitable queue found!");

        println!("Graphics+Present queue at index {}", queue_family_index);

        let priorities = [1.0];

        let queue_info = vk::DeviceQueueCreateInfo {
            queue_family_index,
            p_queue_priorities: priorities.as_ptr(),
            queue_count: 1,
            ..Default::default()
        };

        let device_extensions = [
            ash::khr::swapchain::NAME.as_ptr(),
        ];

        let device_create_info = vk::DeviceCreateInfo {
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_info,
            enabled_extension_count: device_extensions.len() as u32,
            pp_enabled_extension_names: device_extensions.as_ptr(),
            ..Default::default()
        };

        let device = unsafe {
            instance
                .create_device(physical_device, &device_create_info, None)
                .expect("Failed to create logical device")
        };

        let graphics_queue =
            unsafe { device.get_device_queue(queue_family_index, 0) };

        println!("Logical device + queue created!");

        Self {
            physical_device,
            device,
            graphics_queue,
            queue_family_index,
        }
    }
}