struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
}

struct ShaderDataUniforms {
  position: vec2<f32>,
  resolution: vec2<u32>,
  aspect: f32,
  zoom: f32,
  arr_len: i32,
  max_iter: u32,
  color_num: u32,
  flags: u32,
}

@group(0)
@binding(0)
var<uniform> uniforms : ShaderDataUniforms;

@vertex
fn main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vec2<f32>(0.0);
    if in_vertex_index == 0u {
        out.uv = vec2<f32>(-1.0, 1.0);
        out.position = vec4<f32>(-1.0, 1.0, 0.0, 1.0);
    }
    if in_vertex_index == 1u || in_vertex_index == 3u {
        out.uv = vec2<f32>(1.0, 1.0);
        out.position = vec4<f32>(1.0, 1.0, 0.0, 1.0);
    }
    if in_vertex_index == 2u || in_vertex_index == 5u {
        out.uv = vec2<f32>(-1.0, -1.0);
        out.position = vec4<f32>(-1.0, -1.0, 0.0, 1.0);
    }
    if in_vertex_index == 4u {
        out.uv = vec2<f32>(1.0, -1.0);
        out.position = vec4<f32>(1.0, -1.0, 0.0, 1.0);
    }
    return out;
}
