use std::sync::Arc;

use crevice::std140::AsStd140;
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
    ) -> Arc<CpuAccessibleBuffer<[Std140BufferedPointLight]>> {
        let b = BufferedPointLight::from_light(&lights[0]).as_std140();
        println!("{:?}", b);
        CpuAccessibleBuffer::from_iter(
            allocator,
            BufferUsage {
                storage_buffer: true,
                ..BufferUsage::default()
            },
            false,
            lights
                .iter()
                .map(|light| BufferedPointLight::from_light(light).as_std140()),
        )
        .unwrap()
    }
}

#[derive(AsStd140)]
pub struct BufferedPointLight {
    position: Point3,
    color: Vec3,
    intensity: f32,
}
impl BufferedPointLight {
    pub fn from_light(light: &PointLight) -> Self {
        Self {
            position: light.position,
            color: light.color.vec3(),
            intensity: light.intensity,
        }
    }
}
