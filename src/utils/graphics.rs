#![allow(clippy::cast_possible_truncation)]
use std::sync::Mutex;

use crate::{grimoire, structs::rendering::PipelineBufers, Fractals};
use wgpu::{include_wgsl, RequestDeviceError};

use super::GpuStructs;

fn color_raw(color: &wgpu::Color) -> Vec<f32> {
    vec![color.r, color.g, color.b, color.a]
        .iter()
        .map(|i| *i as f32)
        .collect()
}
pub fn to_raw_colors(colors: &[wgpu::Color]) -> Vec<f32> {
    colors.iter().flat_map(color_raw).collect()
}
pub async fn get_device() -> Result<GpuStructs, RequestDeviceError> {
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

pub fn generate_pipeline(fractal: &Fractals, device: &wgpu::Device) -> PipelineBufers {
    //Have the same vertex shader for all fractals
    let vertex = device.create_shader_module(include_wgsl!("../shaders/vert.wgsl"));

    let fragment = device.create_shader_module(match fractal {
        Fractals::Mandebrot => include_wgsl!("../shaders/madelbrot.wgsl"),
        Fractals::Custom(_) => include_wgsl!("../shaders/frag_test.wgsl"),
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
