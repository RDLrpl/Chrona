use std::sync::Arc;

use chrona_vk::vkinit::{devices::GpuDevices, framecontext::FrameContext, pipeline::Executor, render::Render};
use vulkano::instance::Instance;


pub struct AppConfiguration {
    pub width: u32,
    pub height: u32,
    pub fullscreen: bool,
    pub app_version: [u32; 3],
    pub projname: String,
}

pub struct AppState {
    pub vk_instance: Arc<Instance>,

    pub gpudevices: GpuDevices,
    pub render: Render,
    pub executor: Executor,
    pub framecontext: FrameContext,

    pub moving: bool,
}

pub struct AppData {
    pub models_paths: Vec<String>
}

impl AppData {
    pub fn load(models_paths: Vec<String>) -> Self{
        Self {
            models_paths
        }
    }
}

impl AppState {
    pub fn init(vk_instance: Arc<Instance>, gpudevices: GpuDevices, render: Render, executor: Executor, framecontext: FrameContext) -> Self {
        Self { 
            vk_instance, 
            gpudevices,
            render,
            executor,
            framecontext,
            moving:false,
        }
    }
}

impl AppConfiguration {
    pub fn new(projname: &str, app_version: [u32; 3], width: u32, height: u32, fullscreen: bool) -> Self {
        Self {
            projname: projname.to_string(),
            app_version,
            width,
            height,
            fullscreen
        }
    }
}