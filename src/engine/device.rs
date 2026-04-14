se ash::{vk, Instance};

pub struct Device { 
    pub physical_devices= vk::PhysicalDevices,
}     

impl Device {
    pub fn new(instance: &Instance) -> Self {
        let physical_devices = unsafe {
            instance 
                .enumerate_physical_devices()
                .expect("Failed to get GPUs)
        };
        
        if physical_devices.is_empty() {
           painc!("No GPU found!");
        }
        println!("Found {} GPU(s)", physical_devices.len());
        
        let physical_devices = physical_devices[0];
        
        println!("Using first GPU");
          
        Self { physical_device }
     }
}

