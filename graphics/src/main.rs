#![allow(unused)]
use anyhow::{Result,Context};
use ash::{self, vk};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
#[allow(unused_variables)]
fn main() -> Result<()> {
    let entry = unsafe { ash::Entry::load() }?;
    //The VkInstanceCreateInfo structure is used to specify which validation layers and global extensions to                                                                                  // use when creating a Vulkan instance
    let instance = {
        let application_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
        let create_info = vk::InstanceCreateInfo::builder().application_info(&application_info);
        unsafe { entry.create_instance(&create_info, None) }?
};
    let physcial_device = unsafe {
        instance.enumerate_physical_devices()
    }?.into_iter().next().context("NO Physical device found");
    let device = unsafe {
        instance.create_device(physical_device, &create_info, None)
    }?;
    unsafe { instance.destroy_instance(None) }
    Ok(())
}
