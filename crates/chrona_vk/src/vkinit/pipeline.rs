use std::sync::Arc;

use chrona_utils::binding::ResultExt;
use vulkano::{command_buffer::allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, device::Device, pipeline::{GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{ColorBlendAttachmentState, ColorBlendState}, depth_stencil::{DepthState, DepthStencilState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}}, layout::PipelineDescriptorSetLayoutCreateInfo}, render_pass::{RenderPass, Subpass}};

use crate::{pipelines::{fragmentshader, vertexshader}};

use vulkano::{buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}};


#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct VertexDat {
    #[format(R32G32B32_SFLOAT)]
    pub position: [f32; 3],
    #[format(R32G32_SFLOAT)]
    pub uv: [f32; 2],
    #[format(R32G32B32_SFLOAT)]
    pub color: [f32; 3],
}

#[derive(BufferContents)]
#[repr(C)]
pub struct PushConstants {
    pub transform: [[f32; 4]; 4],
}

pub struct Executor {
    pub vertex_buffer: Subbuffer<[VertexDat]>,
    pub pipeline: Arc<GraphicsPipeline>,

    pub viewport: Viewport,
    pub cmd_allocator: Arc<StandardCommandBufferAllocator>,
}

impl Executor {
    pub fn init(memory_allocator: Arc<StandardMemoryAllocator>, vertexes: Vec<VertexDat>, device: Arc<Device>, render_pass: Arc<RenderPass>, viewport: Viewport) -> Self {

        let vertex_buffer = Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            vertexes,
        )
        .unwrap();

        let pipeline = gen_pipeline(device.clone(), render_pass, viewport.clone());

        let cmd_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device,
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        Self {  
            vertex_buffer,
            pipeline,
            cmd_allocator,
            viewport
        }
    }
}



pub fn gen_pipeline(
    device: Arc<Device>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
) -> Arc<GraphicsPipeline> {
    let vs = vertexshader::load(device.clone()).unwrap().entry_point("main").unwrap();
    let fs = fragmentshader::load(device.clone()).unwrap().entry_point("main").unwrap();

    let vertex_input_state = VertexDat::per_vertex().definition(&vs).unwrap();
                      
    let stages = [
        PipelineShaderStageCreateInfo::new(vs),
        PipelineShaderStageCreateInfo::new(fs),
    ];

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    ).unwrap();

    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();

    GraphicsPipeline::new(
        device.clone(),
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState {
                viewports: [viewport].into_iter().collect(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            depth_stencil_state: Some(DepthStencilState {
                depth: Some(DepthState::simple()),
                ..Default::default()
            }),
            color_blend_state: Some(ColorBlendState::with_attachment_states(
                subpass.num_color_attachments(),
                ColorBlendAttachmentState::default(),
            )),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    ).expect_me("[CHRONA]: Pipeline'panic>")
    
}
