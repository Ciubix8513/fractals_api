const max_dot = 200000.0;

fn complex_square(z: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
}

fn complex_div(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    let denumenator = 1.0 / (b.x * b.x + b.y * b.y);
    //Multiplying should be a bit faster
    return vec2<f32>((a.x * b.x + a.y * b.y) * denumenator, (a.y * b.x - a.x * b.y) * denumenator);
}

fn fractal_func(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    if c.x < -1.34 || c.x > 4.0 || abs(c.y) > 1.65 {
        return vec2<f32>(69.0, 4200.0);
    }
    return complex_square(complex_div(z, c)) + c;
}
