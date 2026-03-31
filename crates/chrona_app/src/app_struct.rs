use std::sync::Arc;

use chrona_utils::data::WorldData;
use chrona_vk::{vkinit::{devices::GpuDevices, framecontext::FrameContext, pipeline::Executor, render::Render}};
use vulkano::instance::Instance;

use crate::app::App;


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
    pub world: WorldData, 
    pub func:  GameFunc,
}

pub struct GameFunc {
    pub on_update: fn(&mut App),
}

impl GameFunc {
    pub fn load(on_update: fn(&mut App)) -> Self {
        Self {
            on_update
        }
    }
}

impl AppData {
    pub fn load(world: WorldData, func: GameFunc) -> Self{
        Self {
            world,
            func
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