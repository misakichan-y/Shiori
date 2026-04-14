use ash::{Entry, Instance, vk};
use std::ffi::CString;

pub struct VulkanInstance {
    pub entry: Entry,
    pub instance: Instance,
}

impl VulkanInstance {
     pub fn new() -> Self {
         let entry = unsafe { Entry::load().unwrap() };
 
         let app_name = CString::new("Shiori").unwrap();
         let engine_name = CString::new("Sakura").unwrap();

         let app_info = vk::ApplicationInfo {
             p_application_name: app_name.as_ptr(),
             application_version: 0,
             p_engine_name: engine_name.as_ptr(),
             engine_version: 0,
             api_version: vk::make_api_version(0, 1, 0, 0),
             ..Default::default()
         };
         
         let extentions = [ 
             ash::khr::surface::NAME.as_ptr(),
             ash::khr::xlib_surface::NAME.as_ptr(),
         ];
  
         let create_info = vk::InstanceCreateInfo {
             p_application_info: &app_info,
             enabled_extension_count: extentions.len() as u32,
             pp_enabled_extension_names: extentions.as_ptr(), 
            ..Default::default()
         };    
 
         let instance = unsafe {
             entry
                  .create_instance(&create_info, None)
                  .expect("Bwaaaaa! Check The Code You DUMB")
         };
         
         Self {entry, instance}
    }
}
