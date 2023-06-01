#![allow(dead_code)]

pub const DEFAULT_WIDTH: u32 = 1920;
pub const DEFAULT_HEIGHT: u32 = 1080;

pub const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8Unorm;
pub const CLEAR_COLOR: wgpu::Color = wgpu::Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
pub const STAGING_BELT_SIZE: u64 = 2048;

pub const MAX_COLORS: u64 = 512;

//Flags
pub const SMOOTH_FLAG: u32 = 2 << 31;
pub const DEBUG_FLAG: u32 = 2 << 30;

pub const DEFAULT_COLORS: [wgpu::Color; 5] = [
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
