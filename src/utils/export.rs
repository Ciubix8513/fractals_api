use std::io::Cursor;

use image::{ImageBuffer, Rgba};

pub fn vec_to_png(
    img: &[u8],
    width: u32,
    height: u32,
    format: image::ImageOutputFormat,
) -> Vec<u8> {
    let img = img
        .chunks_exact(4)
        .map(|i| {
            let mut array = [0; 4];
            array.copy_from_slice(i);
            Rgba(array)
        })
        .collect::<Vec<Rgba<u8>>>();

    let mut image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let mut x = 0;
    let mut y = 0;

    for i in img {
        let pixel = image_buffer.get_pixel_mut(x, y);
        x += 1;
        if x == width {
            x = 0;
            y += 1;
        }
        *pixel = i;
    }
    let mut byte_stream = Vec::new();
    image_buffer
        .write_to(&mut Cursor::new(&mut byte_stream), format)
        .unwrap();
    byte_stream
}
