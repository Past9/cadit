use std::sync::Arc;

use crevice::std140::AsStd140;
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::{
    cgmath_types::{vec3, Vec3},
    Rgb,
};

#[derive(AsStd140, Clone, Debug)]
pub struct DirectionalLight {
    direction: Vec3,
    color: Rgb,
    intensity: f32,
}
impl DirectionalLight {
    pub fn new(direction: Vec3, color: Rgb, intensity: f32) -> Self {
        Self {
            direction,
            color,
            intensity,
        }
    }

    pub fn zero() -> Self {
        Self {
            direction: vec3(0.0, 0.0, 1.0),
            color: Rgb::BLACK,
            intensity: 0.0,
        }
    }

    pub fn buffer(
        allocator: &(impl MemoryAllocator + ?Sized),
        lights: Vec<DirectionalLight>,
    ) -> Arc<CpuAccessibleBuffer<[Std140DirectionalLight]>> {
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
