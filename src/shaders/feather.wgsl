const max_dot = 200000.0;

fn complex_div(a: vec2<f32>, b: vec2<f32>) -> vec2<f32> {
    let denumenator = 1.0 / (b.x * b.x + b.y * b.y);
    //Multiplying should be a bit faster
    return vec2<f32>((a.x * b.x + a.y * b.y) * denumenator, (a.y * b.x - a.x * b.y) * denumenator);
}

fn complex_cube(z: vec2<f32>) -> vec2<f32> {
    let x2 = z.x * z.x;
    let y2 = z.y * z.y;
    return vec2<f32>(z.x * x2 - 3.0 * z.x * y2, 3.0 * x2 * z.y - z.y * y2);
}

fn fractal_func(z: vec2<f32>, c: vec2<f32>) -> vec2<f32> {
    if length(c) < 0.53 {
        return vec2<f32>(69.0, 4200.0);
    }
    return complex_div(complex_cube(z), (vec2<f32>(1.0, 0.0) + (z * z))) + c;
}
