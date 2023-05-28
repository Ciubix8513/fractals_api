#![allow(clippy::unused_async, clippy::too_many_lines)]
use std::{env, io::Cursor};

use actix_web::{middleware, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenvy::dotenv;
use image::{ImageBuffer, Rgba};
use wgpu::{Device, Queue, RequestDeviceError};

pub struct GpuStructs {
    pub queue: Queue,
    pub device: Device,
}

#[actix_web::get("/")]
async fn main_page() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("./index.html"))
}

#[actix_web::get("/test")]
async fn render_image(gpu: Data<GpuStructs>) -> impl Responder {
    let mut encoder = gpu
        .device
        .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

    let width = 1920;
    let height = 1080;

    let texture = gpu.device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Render image texture "),
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
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                        a: 1.0,
                    }),
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
    println!("Mapping the buffer");
    gpu.device.poll(wgpu::Maintain::Wait);

    let img = slice
        .get_mapped_range()
        .iter()
        .copied()
        .collect::<Vec<u8>>()
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
        .write_to(
            &mut Cursor::new(&mut byte_stream),
            image::ImageOutputFormat::Png,
        )
        .unwrap();

    let stream = futures::stream::iter(Some(Ok::<actix_web::web::Bytes, std::io::Error>(
        actix_web::web::Bytes::from(byte_stream),
    )));

    HttpResponse::Ok().streaming(stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let ip = env::var("IP_ADDRESS").expect("Ip adress should be set");
    let port = env::var("PORT")
        .expect("Port must be set")
        .parse()
        .expect("Invalid port number");

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        App::new()
            .service(main_page)
            .data_factory(|| async {
                //Initialize wgpu device
                let instance = wgpu::Instance::default();

                let adapter = instance
                    .request_adapter(&wgpu::RequestAdapterOptionsBase {
                        power_preference: wgpu::PowerPreference::default(),
                        force_fallback_adapter: false,
                        compatible_surface: None,
                    })
                    .await
                    .expect("Unable to get an adapter");

                let (device, queue) = adapter
                    .request_device(&wgpu::DeviceDescriptor::default(), None)
                    .await
                    .unwrap();

                let res: Result<GpuStructs, RequestDeviceError> = Ok(GpuStructs { queue, device });
                res
            })
            .service(render_image)
            .wrap(middleware::Logger::default())
    })
    .bind((ip, port))?
    .run()
    .await
}
