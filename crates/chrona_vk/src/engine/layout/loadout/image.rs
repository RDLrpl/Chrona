use std::sync::Arc;

use vulkano::{buffer::{Buffer, BufferCreateInfo, BufferUsage}, command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, PrimaryCommandBufferAbstract, allocator::StandardCommandBufferAllocator}, device::Queue, format::Format, image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}, sync::GpuFuture};

pub fn upload_texture(
    allocator: Arc<StandardMemoryAllocator>,
    queue: Arc<Queue>,
    img_data: &[u8],
    width: u32,
    height: u32,
    cmd_allocator: Arc<StandardCommandBufferAllocator>, 
) -> Arc<ImageView> {
    let image = Image::new(
        allocator.clone(),
        ImageCreateInfo {
            format: Format::R8G8B8A8_UNORM,
            extent: [width, height, 1],
            usage: ImageUsage::TRANSFER_DST | ImageUsage::SAMPLED,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    ).unwrap();

    let buffer = Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        img_data.iter().cloned(),
    ).unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        cmd_allocator.clone(),
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder.copy_buffer_to_image(vulkano::command_buffer::CopyBufferToImageInfo::buffer_image(
        buffer,
        image.clone(),
    )).unwrap();

    let command_buffer = builder.build().unwrap();
    command_buffer.execute(queue.clone()).unwrap()
        .then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    ImageView::new_default(image).unwrap()
}

pub fn no_texture(
    allocator: Arc<StandardMemoryAllocator>,
    queue: Arc<Queue>,
    cmd_allocator: Arc<StandardCommandBufferAllocator>,
) -> Arc<ImageView> {
    let image = Image::new(
        allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1, 1, 1],
            usage: ImageUsage::SAMPLED | ImageUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo::default(),
    ).unwrap();

    let pixel_data: [u8; 4] = [255, 255, 255, 255];

    let buffer = Buffer::from_iter(
        allocator,
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        pixel_data,
    ).unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        cmd_allocator,
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    ).unwrap();

    builder.copy_buffer_to_image(vulkano::command_buffer::CopyBufferToImageInfo::buffer_image(
        buffer,
        image.clone(),
    )).unwrap();

    let command_buffer = builder.build().unwrap();
    command_buffer.execute(queue).unwrap()
        .then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    ImageView::new_default(image).unwrap()
}