use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::{cgmath_types::Vec3, Color};

#[derive(AsStd140, Clone, Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Color,
    intensity: f32,
}
impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            direction,
            color,
            intensity,
        }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        lights: &[DirectionalLight],
    ) -> Arc<CpuAccessibleBuffer<[Std140DirectionalLight]>> {
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
