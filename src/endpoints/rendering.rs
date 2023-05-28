use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    grimoire,
    utils::{export_vec_to_png, GpuStructs},
};

#[actix_web::get("/test")]
async fn render_image(gpu: Data<GpuStructs>) -> impl Responder {
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let width = grimoire::DEFAULT_WIDTH;
    let height = grimoire::DEFAULT_HEIGHT;

    //I think i need to create a new buffer and texture every time
    let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: None,
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        format: wgpu::TextureFormat::Rgba8Unorm,
        view_formats: &[wgpu::TextureFormat::Rgba8Unorm],
        dimension: wgpu::TextureDimension::D2,
        sample_count: 1,
        mip_level_count: 1,
    });
    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    let buffer = gpu.device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        size: {
            let size = texture.size();
            let format = texture.format();
            u64::from(size.width * size.height * format.block_size(None).unwrap())
        },
        mapped_at_creation: false,
    });

    let command_buffer = {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render image pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(grimoire::CLEAR_COLOR),
                    store: true,
                },
            })],
            depth_stencil_attachment: None,
        });

        encoder.copy_texture_to_buffer(
            texture.as_image_copy(),
            wgpu::ImageCopyBufferBase {
                buffer: &buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    bytes_per_row: Some(width * 4),
                    rows_per_image: Some(height),
                },
            },
            texture.size(),
        );
        encoder.finish()
    };

    gpu.queue.submit(Some(command_buffer));

    let slice = buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    gpu.device.poll(wgpu::Maintain::Wait);

    let img = slice
        .get_mapped_range()
        .iter()
        .copied()
        .collect::<Vec<u8>>();
    let byte_stream = export_vec_to_png(&img, height, width, image::ImageOutputFormat::Png);

    let stream = futures::stream::iter(Some(Ok::<web::Bytes, std::io::Error>(web::Bytes::from(
        byte_stream,
    ))));

    HttpResponse::Ok().streaming(stream)
}
