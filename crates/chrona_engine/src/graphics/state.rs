use std::sync::Arc;

use chrona_vk::render::{devices::GpuDevices, framecontext::FrameContext, pipeline::Executor, render::Render};
use vulkano::instance::Instance;

pub struct AppState {
    pub vk_instance: Arc<Instance>,
    pub gpudevices: GpuDevices,
    pub render: Render,
    pub executor: Executor,
    pub framecontext: FrameContext,
}