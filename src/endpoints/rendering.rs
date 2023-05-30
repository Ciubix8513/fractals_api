#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::significant_drop_tightening,
    clippy::too_many_lines
)]
use actix_web::{
    web::{self, Data},
    HttpResponse, Responder,
};

use crate::{
    grimoire,
    utils::{
        export,
        graphics::{generate_pipeline, to_raw_colors, ShaderDataUniforms},
        GpuStructs,
    },
    Fractals, PipelineStore,
};

#[actix_web::get("/test")]
async fn render_image(gpu: Data<GpuStructs>, pipelines: Data<PipelineStore>) -> impl Responder {
    //Hardcoding these in this test function
    let fractal = Fractals::Mandebrot;
    let width = grimoire::DEFAULT_WIDTH;
    let height = grimoire::DEFAULT_HEIGHT;

    let mut pipelines = pipelines.lock().unwrap();

    if !pipelines.contains_key(&fractal) {
        println!("Generating new pipeline for {fractal:#?}");
        pipelines.insert(fractal.clone(), generate_pipeline(&fractal, &gpu.device));
    }

    let pipeline = pipelines.get(&fractal).unwrap();

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

    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let colors = [
        wgpu::Color {
            r: 85.0 / 255.0,
            g: 205.0 / 255.0,
            b: 252.0 / 255.0,
            a: 1.0,
        },
        wgpu::Color {
            r: 247.0 / 255.0,
            g: 168.0 / 255.0,
            b: 184.0 / 255.0,
            a: 1.0,
        },
        wgpu::Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        },
        wgpu::Color {
            r: 247.0 / 255.0,
            g: 168.0 / 255.0,
            b: 184.0 / 255.0,
            a: 1.0,
        },
        wgpu::Color {
            r: 85.0 / 255.0,
            g: 205.0 / 255.0,
            b: 252.0 / 255.0,
            a: 1.0,
        },
    ];

    let data = ShaderDataUniforms {
        resolution: [width, height],
        aspect: height as f32 / width as f32,
        arr_len: colors.len() as u32,
        num_colors: 200,
        msaa: 1,
        max_iter: 1000,
        position: [-0.75, 0.0],
        zoom: 0.75,
    }
    .raw();

    let mut staging_belt = gpu.staging_belt.lock().unwrap();

    staging_belt
        .write_buffer(
            &mut encoder,
            &pipeline.info_buffer,
            0,
            wgpu::BufferSize::new((data.len() * 4) as wgpu::BufferAddress).unwrap(),
            &gpu.device,
        )
        .copy_from_slice(bytemuck::cast_slice(&data));

    let colors = to_raw_colors(&colors);

    staging_belt
        .write_buffer(
            &mut encoder,
            &pipeline.storage_buffer,
            0,
            wgpu::BufferSize::new((colors.len() * 4) as wgpu::BufferAddress).unwrap(),
            &gpu.device,
        )
        .copy_from_slice(bytemuck::cast_slice(&colors));
    {
        //Clear
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
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

        render_pass.set_bind_group(0, &pipeline.bind_group, &[]);
        render_pass.set_pipeline(&pipeline.pipeline);
        render_pass.draw(0..6, 0..1);
    }
    {
        //Copy contents of render texture to the buffer
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
    }

    let command_buffer = encoder.finish();

    staging_belt.finish();
    gpu.queue.submit(Some(command_buffer));
    staging_belt.recall();

    let slice = buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    gpu.device.poll(wgpu::Maintain::Wait);

    let img = slice
        .get_mapped_range()
        .iter()
        .copied()
        .collect::<Vec<u8>>();
    let byte_stream = export::vec_to_png(&img, width, height, image::ImageOutputFormat::Png);

    let stream = futures::stream::iter(Some(Ok::<web::Bytes, std::io::Error>(web::Bytes::from(
        byte_stream,
    ))));

    HttpResponse::Ok().streaming(stream)
}
