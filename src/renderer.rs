// ZephyrAshRenderer 
// Copyright 2026 Daynlight
// Licensed under the GNU General v3.0.
// See LICENSE file for details.



use winit::window::Window;
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use ash::{self, vk, Entry, Instance};
use std::sync::Arc;



pub struct Renderer {
  entry: Option<Entry>,
  instance: Option<Instance>,
  surface: Option<vk::SurfaceKHR>,
  surface_extensions: Option<ash::khr::surface::Instance>,
  window: Option<Arc<Window>>,
  device: Option<Arc<ash::Device>>,
  queue: Option<Arc<ash::vk::Queue>>,
  swapchain: Option<Arc<ash::vk::SwapchainKHR>>,
}



impl Renderer {
  pub fn new() -> Self {
    Renderer {
      entry: None,
      instance: None,
      surface: None,
      surface_extensions: None,
      window: None,
      device: None,
      queue: None,
      swapchain: None,
    }
  }



  pub fn init_renderer(&mut self, window: Arc<Window>) -> Result<(), Box<dyn std::error::Error>> {
    let entry = unsafe { Entry::load()? };
    
    let raw_display_handle = window.display_handle()?.as_raw();
    let raw_window_handle = window.window_handle()?.as_raw();

    let instance = unsafe { entry.create_instance(
      &vk::InstanceCreateInfo::default()
        .application_info(&vk::ApplicationInfo::default().api_version(vk::API_VERSION_1_3))
        .enabled_extension_names(ash_window::enumerate_required_extensions(raw_display_handle)?
      ), 
      None
    ) }?;

    let surface_extensions = ash::khr::surface::Instance::new(&entry, &instance);
    let surface = unsafe { ash_window::create_surface(&entry, &instance, raw_display_handle, raw_window_handle, None) }?;
    let surface_loader = ash::khr::surface::Instance::new(&entry, &instance);

    let physical_devices = unsafe { instance.enumerate_physical_devices() }?;
    let (physical_device, queue_family_index) = physical_devices
      .iter()
      .find_map(|&p_device| {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(p_device) };
        let index = queue_families.iter().enumerate().position(|(i, q_prop)| {
          let supports_graphics = q_prop.queue_flags.contains(vk::QueueFlags::GRAPHICS);
          let supports_surface = unsafe {
            surface_loader
              .get_physical_device_surface_support(p_device, i as u32, surface)
              .unwrap_or(false)
          };   
          supports_graphics && supports_surface
        });
        index.map(|i| (p_device, i as u32))
      })
      .expect("No suitable physical device found");

    let device_extension_names_raw = [ ash::khr::swapchain::NAME.as_ptr() ];

    let queue_priorities = [1.0f32];
    let queue_create_infos = [
      vk::DeviceQueueCreateInfo::default()
        .queue_family_index(queue_family_index)
        .queue_priorities(&queue_priorities)
    ];

    let device_create_info = vk::DeviceCreateInfo::default()
      .queue_create_infos(&queue_create_infos)
      .enabled_extension_names(&device_extension_names_raw);

    let device = unsafe { instance.create_device(physical_device, &device_create_info, None)? };
    let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
    
    let surface_capabilities = unsafe { surface_loader.get_physical_device_surface_capabilities(physical_device, surface)? };
    let surface_formats = unsafe { surface_loader.get_physical_device_surface_formats(physical_device, surface)? };
  
    let surface_format = surface_formats
      .iter()
      .cloned()
      .find(|f| f.format == vk::Format::B8G8R8A8_UNORM && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR)
      .unwrap_or(surface_formats[0]);

    let window_size = window.inner_size();
    let extent = vk::Extent2D {
      width: window_size.width.clamp(
        surface_capabilities.min_image_extent.width,
        surface_capabilities.max_image_extent.width,
      ),
      height: window_size.height.clamp(
        surface_capabilities.min_image_extent.height,
        surface_capabilities.max_image_extent.height,
      ),
    };

    let image_count = (surface_capabilities.min_image_count + 1).clamp(
      surface_capabilities.min_image_count,
      if surface_capabilities.max_image_count > 0 {
        surface_capabilities.max_image_count
      } else {
        u32::MAX
      },
    );

    let swapchain_create_info = vk::SwapchainCreateInfoKHR::default()
      .surface(surface)
      .min_image_count(image_count)
      .image_format(surface_format.format)
      .image_color_space(surface_format.color_space)
      .image_extent(extent)
      .image_array_layers(1)
      .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
      .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
      .pre_transform(surface_capabilities.current_transform)
      .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
      .present_mode(vk::PresentModeKHR::FIFO); 

    let swapchain_device = ash::khr::swapchain::Device::new(&instance, &device);
    let swapchain = unsafe { swapchain_device.create_swapchain(&swapchain_create_info, None)? };    

    self.entry = Some(entry);
    self.instance = Some(instance);
    self.surface = Some(surface);
    self.surface_extensions = Some(surface_extensions);
    self.window = Some(window);
    self.device = Some(Arc::new(device));
    self.queue = Some(Arc::new(queue));
    self.swapchain = Some(Arc::new(swapchain));

    Ok(())
  }



  pub fn on_loop(&self) {

  
  }



  pub fn drop(&mut self) {
     unsafe { self.instance.as_mut().unwrap().destroy_instance(None) };

  }
}
