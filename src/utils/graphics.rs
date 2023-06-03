#![allow(clippy::cast_possible_truncation, clippy::cast_lossless)]
use crate::{
    grimoire,
    structs::rendering::{Fractals, GpuStructs, PipelineBufers},
};
use std::sync::Mutex;
use wgpu::{include_wgsl, RequestDeviceError};

///Flatten `wgpu::Color` into a `[f32; 4]`
pub const fn color_raw(color: &wgpu::Color) -> [f32; 4] {
    [
        color.r as f32,
        color.g as f32,
        color.b as f32,
        color.a as f32,
    ]
}

///Flattens an array of `wgpu::Color` into a `Vec<f32>`
pub fn to_raw_colors(colors: &[wgpu::Color]) -> Vec<f32> {
    colors.iter().flat_map(color_raw).collect()
}

pub fn vec_from_hex(hex: &[&str]) -> Result<Vec<wgpu::Color>, String> {
    hex.iter().map(|h| from_hex(h)).collect()
}

///Converts a hex string ffffffff into `wgpu::Color`
///No hash check bc it's reserved in urls and I don't want to have to input %23
pub fn from_hex(hex: &str) -> Result<wgpu::Color, String> {
    if hex.len() != 6 {
        return Err("Invalid hex color format".to_string());
    }

    let red = u8::from_str_radix(&hex[0..2], 16).map_err(|e| e.to_string())?;
    let green = u8::from_str_radix(&hex[2..4], 16).map_err(|e| e.to_string())?;
    let blue = u8::from_str_radix(&hex[4..6], 16).map_err(|e| e.to_string())?;

    let color = wgpu::Color {
        r: red as f64 / 255.0,
        g: green as f64 / 255.0,
        b: blue as f64 / 255.0,
        a: 1.0,
    };

    Ok(color)
}

///Gets all necessary wgpu structures for the work of the API
pub async fn generate_backend() -> Result<GpuStructs, RequestDeviceError> {
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
        .await?;
    let staging_belt = Mutex::new(wgpu::util::StagingBelt::new(grimoire::STAGING_BELT_SIZE));

    Ok(GpuStructs {
        queue,
        device,
        staging_belt,
    })
}

///Generates a pipeline for rendering a specific type of fractal
pub fn generate_pipeline(fractal: &Fractals, device: &wgpu::Device) -> PipelineBufers {
    log::info!(
        target: grimoire::LOGGING_TARGET,
        "Generating new pipeline for {fractal}"
    );

    //Have the same vertex shader for all fractals
    let vertex = device.create_shader_module(include_wgsl!("../shaders/vert.wgsl"));

    let mut base = include_str!("../shaders/base_fragment.wgsl").to_owned();
    let fractal_fn = match fractal {
        Fractals::Custom(_) | Fractals::Mandelbrot => include_str!("../shaders/madelbrot.wgsl"),
    };
    base.push_str(fractal_fn);
    let fragment = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(base.into()),
    });

    let info_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 10 * 4,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::UNIFORM,
        mapped_at_creation: false,
    });

    let storage_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: 4 * 4 * grimoire::MAX_COLORS,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });

    let bg_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: Some(&format!("{fractal:#?} bind group layout")),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Storage { read_only: true },
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            },
        ],
    });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: None,
        layout: &bg_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: info_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage_buffer.as_entire_binding(),
            },
        ],
    });

    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(&format!("{fractal:#?} pipeline layout")),
        bind_group_layouts: &[&bg_layout],
        push_constant_ranges: &[],
    });
    PipelineBufers {
        pipeline: device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(&format!("{fractal:#?} pipeline")),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &vertex,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fragment,
                entry_point: "main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: grimoire::FORMAT,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multisample: wgpu::MultisampleState::default(),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multiview: None,
        }),
        info_buffer,
        storage_buffer,
        bind_group,
    }
}

#[test]
fn test_hex_to_color() {
    let hex = "ffffff";
    let color = from_hex(hex);
    assert_ne!(color.is_err(), true);
}

#[test]
fn test_hex_vec_to_color() {
    let hex = vec!["ffffff", "ffffff"];
    let color = vec_from_hex(&hex);
    assert_ne!(color.is_err(), true);
}
