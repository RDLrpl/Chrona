use std::sync::Arc;

use tracing::info;
use vulkano::{format::Format, image::{Image, ImageCreateInfo, ImageType, ImageUsage, view::ImageView}, instance::Instance, memory::allocator::{AllocationCreateInfo, StandardMemoryAllocator}, render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass}, swapchain::{Surface, Swapchain, SwapchainCreateInfo}};
use winit::window::Window;

use crate::render::devices::GpuDevices;

#[derive(Clone)]
pub struct Render {
    pub surface: Arc<Surface>,

    pub swapchain: Arc<Swapchain>,
    pub images: Vec<Arc<Image>>,

    pub render_pass: Arc<RenderPass>,
    pub framebuffers: Vec<Arc<Framebuffer>>,

    pub memory_allocator: Arc<StandardMemoryAllocator>
}

impl Render {
    pub fn init(vsync: bool, vk_instance: Arc<Instance>, gpudevices: GpuDevices, window: Arc<Window>) -> Self {
        let surface = Surface::from_window(vk_instance, window.clone()).expect("[CHRONA]: Surface'panic>");

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
                present_mode: if vsync { vulkano::swapchain::PresentMode::Fifo } else { vulkano::swapchain::PresentMode::Immediate },
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
                depth: {
                    format: vulkano::format::Format::D16_UNORM,
                    samples: 1,
                    load_op: Clear,
                    store_op: DontCare,
                }
            },
            pass: {
                color: [color],
                depth_stencil: {depth},
            },
        )
        .expect("[CHRONA]: RENDER_PASS'panic>");

        let depth_images: Vec<Arc<ImageView>> = images.iter().map(|image| {
            let depth_image = Image::new(
                memory_allocator.clone(),
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format: Format::D16_UNORM,
                    extent: image.extent(),
                    usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            ).unwrap();
            ImageView::new_default(depth_image).unwrap()
        }).collect();

        let framebuffers = images.iter().zip(depth_images.iter()).map(|(image, depth_view)| {
            let color_view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![color_view, depth_view.clone()],
                    ..Default::default()
                },
            ).unwrap()
        }).collect::<Vec<_>>();

        info!("VK render initialization completed successfully");

        Self {
            surface,

            swapchain,
            images,

            render_pass,
            framebuffers,

            memory_allocator
        }
    }

    pub fn recreate_swapchain(&mut self, gpudevices: &GpuDevices, window: Arc<Window>) {
        let window_size = window.inner_size();

        if window_size.width == 0 || window_size.height == 0 {
            return;
        }

        unsafe { gpudevices.logical_device.wait_idle().unwrap(); }

        let surface_format = gpudevices.physical_device
            .surface_formats(&self.surface, Default::default())
            .unwrap()[0].0;

        let (new_swapchain, new_images) = match self.swapchain.recreate(SwapchainCreateInfo {
            image_extent: window_size.into(),
            image_format: surface_format,
            ..self.swapchain.create_info()
        }) {
            Ok(r) => r,
            Err(e) => panic!("Failed to recreate swapchain: {e:?}"),
        };

        self.framebuffers.clear();

        let depth_images: Vec<Arc<ImageView>> = new_images.iter().map(|image| {
            let depth_image = Image::new(
                self.memory_allocator.clone(),
                ImageCreateInfo {
                    image_type: ImageType::Dim2d,
                    format: Format::D16_UNORM,
                    extent: image.extent(),
                    usage: ImageUsage::DEPTH_STENCIL_ATTACHMENT,
                    ..Default::default()
                },
                AllocationCreateInfo::default(),
            ).unwrap();
            ImageView::new_default(depth_image).unwrap()
        }).collect();

        let new_framebuffers = new_images.iter().zip(depth_images.iter()).map(|(image, depth_view)| {
            let color_view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                self.render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![color_view, depth_view.clone()],
                    ..Default::default()
                },
            ).unwrap()
        }).collect::<Vec<_>>();

        self.swapchain = new_swapchain;
        self.images = new_images;
        self.framebuffers = new_framebuffers;
    }
}