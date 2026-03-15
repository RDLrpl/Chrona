use std::sync::Arc;

use chrona_utils::binding::ResultExt;
use vulkano::{image::{Image, ImageUsage, view::ImageView}, instance::Instance, memory::allocator::StandardMemoryAllocator, render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass}, swapchain::{Surface, Swapchain, SwapchainCreateInfo}};
use winit::window::Window;

use crate::vkinit::devices::GpuDevices;

pub struct Render {
    pub surface: Arc<Surface>,
    
    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,

    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,

    pub memory_allocator: Arc<StandardMemoryAllocator>
    // pub surfacecapabilities: SurfaceCapabilities,
}

impl Render {
    pub fn init(vk_instance: Arc<Instance>, gpudevices: GpuDevices, window: Arc<Window>) -> Self {
        let surface = Surface::from_window(vk_instance, window.clone()).expect_me("[CHRONA]: Surface'panic>");

        let surface_capabilities = gpudevices.physical_device
            .surface_capabilities(&surface, Default::default())
            .unwrap();

        let surface_format = gpudevices.physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0].0;

        let (swapchain, images) = Swapchain::new(
            gpudevices.logical_device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: surface_capabilities.min_image_count,
                image_format: surface_format,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: surface_capabilities.supported_composite_alpha.into_iter().next().unwrap(),
                ..Default::default()
            },
        ).unwrap();

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(
            gpudevices.logical_device.clone(),
        ));

        let render_pass = vulkano::single_pass_renderpass!(
            gpudevices.logical_device.clone(),
            attachments: {
                color: {
                    format: surface_format,
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .expect_me("[CHRONA]: RENDER_PASS'panic>");
        
        let framebuffers: Vec<Arc<Framebuffer>> = images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .expect_me("[CHRONA]: FrameBuffers'panic>")
            })
            .collect();

        Self {
            surface,

            swapchain,
            images,
            
            render_pass,
            framebuffers,
            
            memory_allocator
        }
    }
}