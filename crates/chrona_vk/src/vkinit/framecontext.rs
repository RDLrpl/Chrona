use std::sync::Arc;

use vulkano::{descriptor_set::allocator::DescriptorSetAllocator, buffer::{BufferUsage, allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo}}, descriptor_set::allocator::StandardDescriptorSetAllocator, memory::allocator::MemoryTypeFilter, sync::GpuFuture};

use crate::vkinit::{devices::GpuDevices, render::Render};


pub struct FrameContext {
    pub previous_frame_end: Option<Box<dyn GpuFuture>>,

    // allocators
    pub descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
    pub uniform_allocator: SubbufferAllocator
}

impl FrameContext { 
    pub fn init(gpudevices: GpuDevices, render: Render) -> Self {
        let previous_frame_end = Some(
            vulkano::sync::now(gpudevices.logical_device.clone()).boxed()
        );

        let descriptor_allocator = Arc::new(StandardDescriptorSetAllocator::new(
            gpudevices.logical_device.clone(),
            Default::default(),
        ));

        let uniform_allocator = SubbufferAllocator::new(
            render.memory_allocator.clone(),
            SubbufferAllocatorCreateInfo {
                buffer_usage: BufferUsage::UNIFORM_BUFFER,
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
        );


        Self {
            previous_frame_end,

            uniform_allocator,
            descriptor_allocator
        }
    }
}