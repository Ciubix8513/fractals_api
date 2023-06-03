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
    structs::{
        rendering::{GpuStructs, ShaderDataUniforms},
        requests::{RequestBody, SimplifiedFractals},
    },
    utils::{
        export,
        graphics::{generate_pipeline, to_raw_colors, vec_from_hex},
    },
    PipelineStore,
};

//Post so that there's a body
///The main endpoint for rendering fractals
#[actix_web::get("/fractals/{fractal}")]
async fn render_fractal(
    gpu: Data<GpuStructs>,
    pipelines: Data<PipelineStore>,
    fractal: web::Path<SimplifiedFractals>,
    query: web::Query<RequestBody>,
) -> impl Responder {
    let fractal = fractal.into_inner();
    //A temporary check while it is not implemented
    if fractal == SimplifiedFractals::Custom {
        return HttpResponse::NotImplemented().into();
    }

    if fractal == SimplifiedFractals::Custom && query.formula.is_none() {
        return HttpResponse::BadRequest()
            .body("Formula should be specified to use custom fractal");
    }

    let fractal = fractal.into_fractals(None);
    let width = query.width.unwrap_or(grimoire::DEFAULT_WIDTH);
    let height = query.height.unwrap_or(grimoire::DEFAULT_HEIGHT);

    let mut pipelines = pipelines.lock().unwrap();

    if !pipelines.contains_key(&fractal) {
        pipelines.insert(fractal.clone(), generate_pipeline(&fractal, &gpu.device));
    }

    let pipeline = pipelines.get(&fractal);
    if pipeline.is_none() {
        log::error!(target: grimoire::LOGGING_TARGET, "Could not get pipeline");
        return HttpResponse::InternalServerError().into();
    }
    let pipeline = pipeline.unwrap();

    //Create texture
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

    let colors = query.colors.clone().map_or_else(
        || Ok(grimoire::DEFAULT_COLORS.into()),
        |v| {
            let v = v.split(',').collect::<Vec<&str>>();
            vec_from_hex(&v)
        },
    );
    if colors.is_err() {
        return HttpResponse::BadRequest().body("Invalid color format");
    }

    let colors = colors.unwrap();

    let data = ShaderDataUniforms {
        aspect: height as f32 / width as f32,
        arr_len: colors.len() as u32,
        max_iter: query.max_iterations.unwrap_or(grimoire::DEFUALT_MAX_ITER),
        num_colors: query.num_colors.unwrap_or(grimoire::DEFAULT_NUM_COLORS),
        zoom: query.zoom.unwrap_or(grimoire::DEFAULT_ZOOM),
        position: [
            query.position_x.unwrap_or(grimoire::DEFAULT_POSITION[0]),
            query.position_y.unwrap_or(grimoire::DEFAULT_POSITION[1]),
        ],
        flags: {
            let mut result = u32::from(query.msaa.unwrap_or(1));
            if query.smooth.unwrap_or_default() {
                result |= grimoire::rendering_flags::SMOOTH;
            }
            if query.debug.unwrap_or_default() {
                result |= grimoire::rendering_flags::DEBUG;
            }
            result
        },
    }
    .raw();

    let staging_belt = gpu.staging_belt.lock();
    if staging_belt.is_err() {
        log::error!(
            target: grimoire::LOGGING_TARGET,
            "Could not get staging belt {}",
            staging_belt.err().unwrap()
        );

        return HttpResponse::InternalServerError().into();
    }
    let mut staging_belt = staging_belt.unwrap();

    let buffer_size = wgpu::BufferSize::new((data.len() * 4) as wgpu::BufferAddress);
    if buffer_size.is_none() {
        log::error!(
            target: grimoire::LOGGING_TARGET,
            "Could not get data buffer size"
        );
        return HttpResponse::InternalServerError().into();
    }

    staging_belt
        .write_buffer(
            &mut encoder,
            &pipeline.info_buffer,
            0,
            buffer_size.unwrap(),
            &gpu.device,
        )
        .copy_from_slice(bytemuck::cast_slice(&data));

    let colors = to_raw_colors(&colors);

    let color_buffer_size = wgpu::BufferSize::new((colors.len() * 4) as wgpu::BufferAddress);
    if buffer_size.is_none() {
        log::error!(
            target: grimoire::LOGGING_TARGET,
            "Could not get color buffer size"
        );
        return HttpResponse::InternalServerError().into();
    }

    staging_belt
        .write_buffer(
            &mut encoder,
            &pipeline.storage_buffer,
            0,
            color_buffer_size.unwrap(),
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
    texture.destroy();

    let slice = buffer.slice(..);
    slice.map_async(wgpu::MapMode::Read, |_| {});
    gpu.device.poll(wgpu::Maintain::Wait);

    let img = slice
        .get_mapped_range()
        .iter()
        .copied()
        .collect::<Vec<u8>>();
    let byte_stream = export::arr_to_image(&img, width, height, image::ImageOutputFormat::Png);

    if byte_stream.is_err() {
        log::error!(
            target: grimoire::LOGGING_TARGET,
            "Could not export image {}",
            byte_stream.err().unwrap()
        );
        return HttpResponse::InternalServerError().body("Unable to export image");
    }

    let stream = futures::stream::iter(Some(Ok::<web::Bytes, std::io::Error>(web::Bytes::from(
        byte_stream.unwrap(),
    ))));

    HttpResponse::Ok().streaming(stream)
}
