struct ShaderDataUniforms {
  position: vec2<f32>,
  aspect: f32,
  zoom: f32,
  arr_len: i32,
  max_iter: u32,
  color_num: u32,
  flags: u32,
}

struct VertexOutput {
  @builtin(position) position: vec4<f32>,
  @location(0)
  uv: vec2<f32>,
}
@group(0)
@binding(0)
var<uniform> uniforms : ShaderDataUniforms;
@group(0)
@binding(1)
var<storage, read>  colors : array<vec4<f32>>;

//I don't remember where I got this, but it should work
fn rand(s: f32) -> f32 {
    return fract(sin(s * 12.9898) * 43758.5453);
}

fn complex_square(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}
fn get_col(coord: f32, col_num: i32) -> vec4<f32> {

    if col_num == 1 {
        return colors[0];
    }
    let cstep1 = 1.0 / f32(col_num - 1);
    for (var i = 1; i < col_num; i += 1) {
        if coord < cstep1 * f32(i) {
            return mix(colors[(i - 1) % uniforms.arr_len], colors[i % uniforms.arr_len], coord / cstep1 - f32(i - 1));
        }
    }
    return vec4<f32>(coord);
}

fn get_color(uv: vec2<f32>, i: f32, max_i: u32) -> vec4<f32> {
    if i >= f32(max_i) {
        return vec4<f32>(0.0, 0.0, 0.0, 1.0);
    }
    return get_col(f32(i) / f32(max_i), i32(uniforms.color_num));
}

fn fractal(C: vec2<f32>) -> vec4<f32> {
    var coords = vec2<f32>(0.0);
    var iter = 0u;

    var max_dot = 5.0;
    let max_iteration = uniforms.max_iter;

    while dot(coords, coords) <= max_dot && iter < max_iteration {
        coords = mandelbrot(coords, C);
        iter += 1u;
    }
    if iter >= max_iteration {
        return vec4<f32>(0.0);
    }

    var i = f32(iter);
    if coords.x == 69.0 && coords.y == 4200.0 {
        return vec4<f32>(0.0);
    } else if (uniforms.flags & (2u << 30u)) != 0u {
        i = i - log2(log2(dot(coords, coords))) + 4.0;
    }
    return get_color(C, i, max_iteration);
}

@fragment
fn main(in: VertexOutput) -> @location(0) vec4<f32> {
    let msaa = uniforms.flags | 255u;

    let uv = (in.uv / vec2<f32>(uniforms.aspect, 1.0));
    let transformed_uv = uv / uniforms.zoom + uniforms.position;
    var col = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    col += fractal(transformed_uv);

    //Display debug info
    if (uniforms.flags & (2u << 29u)) != 0u {
        if length(uv) < 0.025 {
            return vec4<f32>(0.0, 0.0, 1.0, 1.0);
        }
        if (abs(transformed_uv.x) % 0.1) < 0.01 {
            return vec4<f32>(1.0, 0.0, 0.0, 1.0);
        }
        if (abs(transformed_uv.y) % 0.1) < 0.01 {
            return vec4<f32>(0.0, 1.0, 0.0, 1.0);
        }
    }
    return col;
}

fn mandelbrot(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    let c2 = dot(c, c);

    // skip computation inside M1 - https://iquilezles.org/articles/mset1bulb
    if 256.0 * c2 * c2 - 96.0 * c2 + 32.0 * c.x - 3.0 < 0.0 {
        return vec2<f32>(69.0, 4200.0);
    }
    // skip computation inside M2 - https://iquilezles.org/articles/mset2bulb
    if 16.0 * (c2 + 2.0 * c.x + 1.0) - 1.0 < 0.0 {
        return vec2<f32>(69.0, 4200.0);
    }
    return complex_square(z) + c;
}
