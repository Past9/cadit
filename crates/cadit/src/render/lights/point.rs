use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::{
    cgmath_types::{point3, Point3},
    Rgb,
};

#[derive(AsStd140, Clone, Debug)]
pub struct PointLight {
    position: Point3,
    color: Rgb,
    intensity: f32,
}
impl PointLight {
    pub fn new(position: Point3, color: Rgb, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn zero() -> Self {
        Self {
            position: point3(0.0, 0.0, 0.0),
            color: Rgb::BLACK,
            intensity: 0.0,
        }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        lights: Vec<PointLight>,
    ) -> Arc<CpuAccessibleBuffer<[Std140PointLight]>> {
        CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::default()
            },
            false,
            match lights.len() {
                len if len > 0 => lights,
                _ => vec![Self::zero()],
            }
            .into_iter()
            .map(|light| light.as_std140()),
        )
        .unwrap()
    }
}
