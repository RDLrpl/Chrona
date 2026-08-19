use std::{error::Error, sync::Arc};

use vulkano::{device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags, physical::{PhysicalDevice, PhysicalDeviceType}}, instance::Instance};

#[derive(Clone)]
pub struct GpuDevices {
    pub physical_device: Arc<PhysicalDevice>,
    pub logical_device: Arc<Device>,
    pub queue_family_index: u32,
    pub queue: Arc<Queue>,
    pub device_name: String,
}

// GPU init:
impl GpuDevices {
    pub fn init(appinstance: Arc<Instance>, device_exstensions: DeviceExtensions) -> Self {
        // Phys Device
        let (physical_device, device_name) = best_gpu(appinstance, device_exstensions).expect("[ERROR] GPU INIT>").unwrap();

        // choosing GPU GRAPHICS QUEUE (req. logic device)
        let queue_family_index = physical_device
            .queue_family_properties()
            .iter()
            .position(|queue_family_properties| {
                queue_family_properties.queue_flags.contains(QueueFlags::GRAPHICS)
            })
            .expect("[CHRONA]: NO QUEUE'panic>") as u32;

        // logic device
        let (logical_device, mut queues) = Device::new(
            physical_device.clone(),
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_exstensions,
                ..Default::default()
            },
        )
        .expect("[CHRONA]: FAILED TO CREATE LOGICAL GPU'panic>");
        
        // prefer queue
        let queue = queues.next().expect("[CHRONA]: NO QUEUE'panic>");

        Self {
            physical_device,
            logical_device,
            queue_family_index,
            queue,
            device_name,
        }
    }
}

// GPU selection:
fn best_gpu(vk_instance: Arc<Instance>, device_exstensions: DeviceExtensions) -> Result<Option<(Arc<PhysicalDevice>, String)>, Box<dyn Error>> {
    // best gpu choosing version 0.1.0
    let devices = vk_instance.enumerate_physical_devices()?;
    
    let mut best_device: Option<(Arc<PhysicalDevice>, u32)> = None;

    for (_, device) in devices
        .enumerate()
        .filter(|&(_, ref device)| device.supported_extensions().contains(&device_exstensions))
    {

        let props = device.properties();
        let mut current_score: u32 = 0;
        
        let memory_mb = device.memory_properties().memory_heaps.iter()
            .filter(|heap| heap.flags.contains(vulkano::memory::MemoryHeapFlags::DEVICE_LOCAL))
            .map(|heap| heap.size)
            .sum::<u64>() as f64 / 1024.0 / 1024.0;

        if props.device_type == PhysicalDeviceType::DiscreteGpu {
            current_score += 100_000;
        } else if props.device_type == PhysicalDeviceType::IntegratedGpu {
            current_score += 20_000;
        }

        current_score += (memory_mb * 4.5) as u32;


        if best_device.is_none() || current_score > best_device.as_ref().expect("[CHRONA]: NO VALID GPU ![maybe, you have GPU without VK support or smth problems in vkinit>best_gpu(CHRONA)]'panic>").1 {
            best_device = Some((device, current_score));
        }
    }

    let (winner_device, _) = best_device.expect("[CHRONA]: NO VALID GPU ![maybe, you have GPU without VK support]'panic>");

    let devicename = winner_device.properties().device_name.clone();
    
    Ok(Some((winner_device, devicename)))
}
