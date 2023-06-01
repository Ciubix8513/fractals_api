#![allow(dead_code)]
use std::{collections::HashMap, sync::Mutex};

///Stores all need wgpu structs in the api state
pub struct GpuStructs {
    pub queue: wgpu::Queue,
    pub device: wgpu::Device,
    pub staging_belt: Mutex<wgpu::util::StagingBelt>,
}

//I'm gonna hard code the buffers for now, tho it may be a good idea to not do it
pub struct PipelineBufers {
    pub pipeline: wgpu::RenderPipeline,
    pub info_buffer: wgpu::Buffer,
    pub storage_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct ShaderDataUniforms {
    ///Unscaled position of the center of the image
    pub position: [f32; 2],
    //Recalculated so that the gpu doesn't have to do the same work
    //even tho it's probably not a big deal
    ///Aspect ratio of the image (height/width)
    pub aspect: f32,
    ///Scale of the coordinates
    pub zoom: f32,
    ///Length of the color storage buffer
    pub arr_len: u32,
    ///Maximum number of iterations of the fractal
    pub max_iter: u32,
    //TODO make a page for explaining how it works
    ///Number of colored stripes
    pub num_colors: u32,
    ///First 4 bits for msaa, the rest are flags from the grimoire
    pub flags: u32,
}

impl Default for ShaderDataUniforms {
    fn default() -> Self {
        Self {
            aspect: Default::default(),
            arr_len: Default::default(),
            position: [-0.75, 0.0],
            zoom: 1.0,
            max_iter: 1000,
            num_colors: 200,
            flags: 1,
        }
    }
}

impl ShaderDataUniforms {
    pub fn raw(&self) -> [u32; 8] {
        [
            self.position[0].to_bits(),
            self.position[1].to_bits(),
            self.aspect.to_bits(),
            self.zoom.to_bits(),
            self.arr_len,
            self.max_iter,
            self.num_colors,
            self.flags,
        ]
    }
}

///Represents types of fractals that the api can render
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub enum Fractals {
    Mandebrot,
    Custom(String),
}

///A helper type for the api state
pub type PipelineStore = Mutex<HashMap<Fractals, PipelineBufers>>;
