use std::sync::Arc;

use chrona_utils::data::VertexDat;
use chrona_world::engine::{layout::world::world::Scene, shr::ModelPushConstant};
use vulkano::{command_buffer::{AutoCommandBufferBuilder, PrimaryAutoCommandBuffer, allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}}, descriptor_set::{DescriptorSet, WriteDescriptorSet}, device::Device, image::sampler::{Sampler, SamplerCreateInfo}, pipeline::{DynamicState, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo, graphics::{GraphicsPipelineCreateInfo, color_blend::{ColorBlendAttachmentState, ColorBlendState}, depth_stencil::{DepthState, DepthStencilState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}}, layout::PipelineDescriptorSetLayoutCreateInfo}, render_pass::{RenderPass, Subpass}};

use crate::{pipelines::{fragmentshader, vertexshader}, vkinit::framecontext::FrameContext};

use vulkano::{buffer::{BufferContents}};


#[derive(BufferContents)]
#[repr(C)]
pub struct PushConstants {
    pub transform: [[f32; 4]; 4],
}

pub struct Executor {
    pub pipeline: Arc<GraphicsPipeline>,

    pub viewport: Viewport,
    pub cmd_allocator: Arc<StandardCommandBufferAllocator>,
    pub sampler: Arc<Sampler>, 
}

impl Executor {
    pub fn init(
        device: Arc<Device>, 
        render_pass: Arc<RenderPass>,
        viewport: Viewport
    ) -> Self {

        let pipeline = gen_pipeline(device.clone(), render_pass);

        let cmd_allocator = Arc::new(StandardCommandBufferAllocator::new(
            device.clone(),
            StandardCommandBufferAllocatorCreateInfo::default(),
        ));

        let sampler = Sampler::new(
            device,
            SamplerCreateInfo::simple_repeat_linear(),
        ).unwrap();

        Self {  
            pipeline,
            cmd_allocator,
            viewport,
            sampler
        }
    }

    pub fn draw(&self, builder: &mut AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>, scene: &Scene, frame_ctx: &FrameContext,) {
        for model in &scene.models {
            let layout = self.pipeline.layout().set_layouts()[1].clone();

            let descriptor_set = DescriptorSet::new(
                frame_ctx.descriptor_allocator.clone(),
                layout,
                [WriteDescriptorSet::image_view_sampler(0, model.texture.clone(), self.sampler.clone())],
                [],
            ).unwrap();

            builder.bind_descriptor_sets(
                PipelineBindPoint::Graphics,
                self.pipeline.layout().clone(),
                1,
                descriptor_set,
            ).unwrap();
            
            let push = ModelPushConstant {
                model: model.transf.to_model_matrix().to_cols_array_2d(),
            };

            unsafe {
                builder
                    .push_constants(self.pipeline.layout().clone(), 0, push).unwrap()
                    .bind_vertex_buffers(0, model.vertex_buffer.clone()).unwrap()
                    .draw(model.vertex_buffer.len() as u32, 1, 0, 0)
                    .expect("[CHRONA]: Draw Model'panic>");
            }
        }
    }

}

pub fn gen_pipeline(
    device: Arc<Device>,
    render_pass: Arc<RenderPass>
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
            viewport_state: Some(ViewportState::default()),
            dynamic_state: [DynamicState::Viewport].into_iter().collect(),
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
    ).expect("[CHRONA]: Pipeline'panic>")
    
}
