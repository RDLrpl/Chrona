use std::sync::Arc;

use vulkano::{buffer::{BufferUsage, Subbuffer, allocator::{SubbufferAllocator, SubbufferAllocatorCreateInfo}}, descriptor_set::{DescriptorSet, WriteDescriptorSet, allocator::{DescriptorSetAllocator, StandardDescriptorSetAllocator}}, memory::allocator::MemoryTypeFilter, pipeline::PipelineLayout, sync::{GpuFuture, future::FenceSignalFuture}};

use crate::{pipelines::vertexshader::CameraUBO, render::{devices::GpuDevices, render::Render}};


pub struct FrameContext {
    pub frame_fences: Vec<Option<FenceSignalFuture<Box<dyn GpuFuture>>>>,

    // allocators
    pub descriptor_allocator: Arc<dyn DescriptorSetAllocator>,
    pub uniform_allocator: SubbufferAllocator,

    pub uniform_subbuffers: Vec<Subbuffer<CameraUBO>>,
    pub camera_descriptors: Vec<Arc<DescriptorSet>>,
}

impl FrameContext { 
    pub fn init(gpudevices: GpuDevices, render: Render, pipeline_layout: Arc<PipelineLayout>) -> Self {
        let mut frame_fences: Vec<Option<FenceSignalFuture<Box<dyn GpuFuture>>>> = Vec::with_capacity(3);
        for _ in 0..3 {
            frame_fences.push(None);
        }

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

        let layout = pipeline_layout.set_layouts()[0].clone();

        let mut uniform_subbuffers = Vec::with_capacity(3);
        let mut camera_descriptors = Vec::with_capacity(3);

        for _ in 0..3 {
            let buf = uniform_allocator.allocate_sized::<CameraUBO>().unwrap();

            let descriptor = DescriptorSet::new(
                descriptor_allocator.clone(),
                layout.clone(),
                [WriteDescriptorSet::buffer(0, buf.clone())],
                [],
            ).unwrap();

            uniform_subbuffers.push(buf);
            camera_descriptors.push(descriptor);
        }
        Self {
            frame_fences,

            uniform_allocator,
            descriptor_allocator,
            uniform_subbuffers, 
            camera_descriptors
        }
    }
}