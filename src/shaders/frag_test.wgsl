
struct ShaderDataUniforms {
  position: vec2<f32>,
  resolution: vec2<u32>,
  aspect: f32,
  zoom: f32,
  arr_len: i32,
  max_iter: u32,
  color_num: u32,
  msaa: u32,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0)
  uv: vec2<f32>,
}

@group(0)
@binding(0)
var<uniform> uniforms : ShaderDataUniforms;

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    var uv = in.uv / vec2<f32>(uniforms.resolution);
    return vec4<f32>(uv.xy, 0.0, 1.0);
}
