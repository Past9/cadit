use bytemuck::{Pod, Zeroable};
use vulkano::{buffer::CpuAccessibleBuffer, memory::allocator::MemoryAllocator};

use crate::render::{cgmath_types::Vec3, Color};

pub struct PointLight {
    position: Vec3,
    color: Color,
    intensity: f32,
}
impl PointLight {
    pub fn new(position: Vec3, color: Color, intensity: f32) -> Self {
        Self {
            position,
            color,
            intensity,
        }
    }

    pub fn buffer<I>(allocator: &(impl MemoryAllocator + ?Sized), lights: I)
    where
        I: IntoIterator<Item = PointLight>,
        I::IntoIter: ExactSizeIterator,
    {
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone, Zeroable, Pod)]
struct BufferedPointLight {
    position: [f32; 4],
    color: [f32; 4],
    intensity: f32,
}
impl From<PointLight> for BufferedPointLight {
    fn from(light: PointLight) -> Self {
        Self {
            position: [light.position.x, light.position.y, light.position.z, 1.0],
            color: *light.color.bytes(),
            intensity: light.intensity,
        }
    }
}
