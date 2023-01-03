use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::Color;

#[derive(AsStd140, Clone, Debug)]
pub struct AmbientLight {
    color: Color,
    intensity: f32,
}
impl AmbientLight {
    pub fn new(color: Color, intensity: f32) -> Self {
        Self { color, intensity }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        lights: &[AmbientLight],
    ) -> Arc<CpuAccessibleBuffer<[Std140AmbientLight]>> {
        CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::default()
            },
            false,
            lights.iter().map(|light| light.as_std140()),
        )
        .unwrap()
    }
}
