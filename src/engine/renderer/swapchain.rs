use ash::{vk, Instance};
use crate::engine::renderer::device::Device;
use crate::engine::renderer::surface::Surface;

pub struct Swapchain {
    pub loader: ash::khr::swapchain::Device, 
    pub swapchain: vk::SwapchainKHR,
    pub images: Vec<vk::Image>,
    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl Swapchain { 
    pub fn new(
        instance: &Instance,
        device: &Device,
        surface: &Surface,
    ) -> Self { 
         let surface_caps = unsafe { 
             surface 
                 .loader 
                 .get_physical_device_surface_capabilities(
                      device.physical_device,
                      surface.surface,
                 ) 
                 .expect("Failed to get surface capabilities")
          };
          let formats = unsafe {
              surface
                  .loader
                  .get_physical_device_surface_formats(
                      device.physical_device,
                      surface.surface,
              )
              .expect("Failed too get surface formats")
          };
 
          let surface_format = formats[0];
          
          let extent = surface_caps.current_extent;
          
          let image_count = surface_caps.min_image_count + 1;
     
          let create_info = vk::SwapchainCreateInfoKHR {
              surface: surface.surface,
              min_image_count: image_count,
              image_format: surface_format.format,
              image_color_space: surface_format.color_space,
              image_extent: extent,
              image_array_layers: 1,
              image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
              image_sharing_mode: vk::SharingMode::EXCLUSIVE,
              pre_transform: surface_caps.current_transform,
              composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
              clipped: vk::TRUE,
              ..Default::default()
        };

        let loader = ash::khr::swapchain::Device::new(
            &instance,
            &device.device,
        );
        let swapchain = unsafe {
            loader
            .create_swapchain(&create_info, None)
            .expect("Failed to create swapchain")
        };

        let images = unsafe {
            loader 
              .get_swapchain_images(swapchain)
              .expect("Failed to get swapchain images")
        };

        println!("Swapchain created with {} images", images.len());

        Self {
            loader,
            swapchain,
            images,
            format: surface_format.format,
            extent,
        }
    }
}
