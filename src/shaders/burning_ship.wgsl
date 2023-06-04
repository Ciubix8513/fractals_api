fn complex_square(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

fn fractal_func(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    return complex_square(abs(z)) + c;
}
