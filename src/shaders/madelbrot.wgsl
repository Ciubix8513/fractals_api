const max_dot = 4.0;

fn complex_square(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

fn fractal_func(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
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
