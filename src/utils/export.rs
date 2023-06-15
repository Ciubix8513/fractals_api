use std::io::Cursor;

use actix_web::web::Bytes;
use image::{ImageBuffer, ImageError, Rgba};

use crate::grimoire;

///Transforms an array of raw image bytes into a specified format
pub fn arr_to_image(
    img: &[u8],
    bytes_per_row: u32,
    width: u32,
    height: u32,
    format: image::ImageOutputFormat,
) -> Result<Vec<u8>, ImageError> {
    let img = img
        .chunks_exact(4)
        .map(|i| {
            let mut array = [0; 4];
            array.copy_from_slice(i);
            Rgba(array)
        })
        .collect::<Vec<Rgba<u8>>>();

    log::debug!(target: grimoire::LOGGING_TARGET, "Collected the array");
    log::debug!(
        target: grimoire::LOGGING_TARGET,
        "BPR = {bytes_per_row}, width = {width}"
    );
    let mut image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
    let mut x = 0;
    let mut y = 0;

    for i in img {
        //Remove padding
        if width != bytes_per_row && x >= width {
            x += 1;
            if x == bytes_per_row {
                x = 0;
                y += 1;
            }
            continue;
        }

        *image_buffer.get_pixel_mut(x, y) = i;

        x += 1;
        //This is kind of a hacky fix, but it works
        if x == width && width == bytes_per_row {
            x = 0;
            y += 1;
        }
    }
    log::debug!(
        target: grimoire::LOGGING_TARGET,
        "Iterated through the array"
    );
    let mut byte_stream = Vec::new();
    image_buffer.write_to(&mut Cursor::new(&mut byte_stream), format)?;

    Ok(byte_stream)
}

pub fn async_iter(
    arr: Vec<u8>,
) -> futures::stream::Iter<std::option::IntoIter<Result<Bytes, std::io::Error>>> {
    futures::stream::iter(Some(Ok::<Bytes, std::io::Error>(Bytes::from(arr))))
}
