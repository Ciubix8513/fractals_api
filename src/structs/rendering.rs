//I'm gonna hard code the buffers for now, tho it may be a good idea to not do it
pub struct PipelineBufers {
    pub pipeline: wgpu::RenderPipeline,
    pub info_buffer: wgpu::Buffer,
    pub storage_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct ShaderDataUniforms {
    pub position: [f32; 2],
    pub aspect: f32,
    pub zoom: f32,
    pub arr_len: u32,
    pub max_iter: u32,
    pub num_colors: u32,
    pub msaa: u32,
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
            self.msaa,
        ]
    }
}
