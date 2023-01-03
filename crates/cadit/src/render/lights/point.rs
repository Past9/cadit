use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::{cgmath_types::Point3, Color};

#[derive(AsStd140, Clone, Debug)]
pub struct PointLight {
    position: Point3,
    color: Color,
    intensity: f32,
}
impl PointLight {
    pub fn new(position: Point3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        lights: &[PointLight],
    ) -> Arc<CpuAccessibleBuffer<[Std140PointLight]>> {
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
