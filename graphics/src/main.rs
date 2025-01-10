#![allow(unused)]
use anyhow::Result;
use ash::{self, vk};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
#[allow(unused_variables)]
fn main() -> Result<()> {
    let entry = unsafe { ash::Entry::load() }?;
    let application_info = vk::ApplicationInfo::builder().api_version(vk::API_VERSION_1_3);
    let create_info = vk::InstanceCreateInfo::builder().application_info(&application_info); //The VkInstanceCreateInfo structure is used to specify which validation layers and global extensions to                                                                                  // use when creating a Vulkan instance
    let instance = unsafe { entry.create_instance(&create_info, None) }?;
    unsafe { instance.destroy_instance(None) }
    Ok(())
}
