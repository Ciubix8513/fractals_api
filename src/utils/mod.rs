use std::sync::Mutex;

use wgpu::{util::StagingBelt, Device, Queue};

pub struct GpuStructs {
    pub queue: Queue,
    pub device: Device,
    pub staging_belt: Mutex<StagingBelt>,
}

pub mod export;
pub mod graphics;
