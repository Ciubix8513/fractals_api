struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0) uv: vec2<f32>,
}

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
