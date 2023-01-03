use std::sync::Arc;

use bytemuck::{Pod, Zeroable};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    memory::allocator::MemoryAllocator,
};

use crate::render::{
    cgmath_types::{Point3, Vec3},
    Color,
};

#[derive(Clone, Debug)]
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
    ) -> Arc<CpuAccessibleBuffer<[BufferedPointLight]>> {
        CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::default()
            },
            false,
            lights
                .iter()
                .to_owned()
                .map(|light| BufferedPointLight::from(light.clone())),
        )
        .unwrap()
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
pub struct BufferedPointLight {
    position: [f32; 4],
    color: [f32; 4],
    intensity: f32,
    a: f32,
    b: f32,
    c: f32,
}
impl From<PointLight> for BufferedPointLight {
    fn from(light: PointLight) -> Self {
        Self {
            position: [light.position.x, light.position.y, light.position.z, 1.0],
            color: *light.color.bytes(),
            intensity: light.intensity,
            a: 0.0,
            b: 0.0,
            c: 0.0,
        }
    }
}
